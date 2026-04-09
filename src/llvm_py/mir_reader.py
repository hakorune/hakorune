"""
MIR JSON Reader
Parses Nyash MIR JSON format into Python structures
"""

from dataclasses import dataclass
from typing import Callable, Dict, List, Any, Optional, Union, Tuple
from enum import Enum

class MirType(Enum):
    """MIR type enumeration"""
    VOID = "void"
    I64 = "i64"
    F64 = "f64"
    BOOL = "bool"
    STRING = "string"
    BOX = "box"
    ARRAY = "array"
    MAP = "map"
    PTR = "ptr"

@dataclass
class MirFunction:
    """MIR function representation"""
    name: str
    params: List[Tuple[str, MirType]]
    return_type: MirType
    blocks: Dict[int, 'MirBlock']
    entry_block: int

@dataclass
class MirBlock:
    """MIR basic block"""
    id: int
    instructions: List['MirInstruction']
    terminator: Optional['MirInstruction']

@dataclass
class MirInstruction:
    """Base MIR instruction"""
    kind: str
    
    # Common fields
    dst: Optional[int] = None
    
    # Instruction-specific fields
    value: Optional[Any] = None  # For Const
    op: Optional[str] = None     # For BinOp/Compare
    lhs: Optional[int] = None    # For BinOp/Compare
    rhs: Optional[int] = None    # For BinOp/Compare
    cond: Optional[int] = None   # For Branch
    then_bb: Optional[int] = None
    else_bb: Optional[int] = None
    target: Optional[int] = None # For Jump
    box_val: Optional[int] = None # For BoxCall
    method: Optional[str] = None
    args: Optional[List[int]] = None


@dataclass
class BuilderInput:
    """Normalized MIR payload for llvm_builder orchestration."""
    user_box_decls: List[Dict[str, Any]]
    enum_decls: List[Dict[str, Any]]
    functions: List[Dict[str, Any]]
    call_arities: Dict[str, int]
    
def parse_mir_json(data: Dict[str, Any]) -> Dict[str, MirFunction]:
    """Parse MIR JSON into Python structures"""
    functions = {}
    
    # Parse each function
    for func_name, func_data in data.get("functions", {}).items():
        # Parse parameters
        params = []
        for param in func_data.get("params", []):
            params.append((param["name"], MirType(param["type"])))
        
        # Parse blocks
        blocks = {}
        for block_id, block_data in func_data.get("blocks", {}).items():
            bid = int(block_id)
            
            # Parse instructions
            instructions = []
            for instr_data in block_data.get("instructions", []):
                instr = parse_instruction(instr_data)
                instructions.append(instr)
            
            # Parse terminator
            terminator = None
            if "terminator" in block_data:
                terminator = parse_instruction(block_data["terminator"])
            
            blocks[bid] = MirBlock(bid, instructions, terminator)
        
        # Create function
        func = MirFunction(
            name=func_name,
            params=params,
            return_type=MirType(func_data.get("return_type", "void")),
            blocks=blocks,
            entry_block=func_data.get("entry_block", 0)
        )
        
        functions[func_name] = func
    
    return functions

def parse_instruction(data: Dict[str, Any]) -> MirInstruction:
    """Parse a single MIR instruction"""
    kind = data["kind"]
    instr = MirInstruction(kind=kind)
    
    # Copy common fields
    for field in ["dst", "value", "op", "lhs", "rhs", "cond", 
                  "then_bb", "else_bb", "target", "box_val", "method"]:
        if field in data:
            setattr(instr, field, data[field])
    
    # Handle args array
    if "args" in data:
        instr.args = data["args"]
    
    return instr


def normalize_functions_payload(mir_json: Dict[str, Any]) -> List[Dict[str, Any]]:
    """Normalize v0/v1 MIR `functions` payload into llvm_builder list form."""
    funcs = mir_json.get("functions", [])
    if isinstance(funcs, list):
        return funcs

    normalized: List[Dict[str, Any]] = []
    if isinstance(funcs, dict):
        for name, func_data in funcs.items():
            entry = dict(func_data)
            entry["name"] = name
            normalized.append(entry)
    return normalized


def build_builder_input(
    mir_json: Dict[str, Any],
    scan_call_arities_fn: Callable[[List[Dict[str, Any]]], Dict[str, int]],
) -> BuilderInput:
    """Collect llvm_builder ingest payload in one owner-local seam."""
    functions = normalize_functions_payload(mir_json)
    try:
        call_arities = scan_call_arities_fn(functions)
    except Exception:
        call_arities = {}

    user_box_decls = mir_json.get("user_box_decls", [])
    if not isinstance(user_box_decls, list):
        user_box_decls = []
    enum_decls = mir_json.get("enum_decls", [])
    if not isinstance(enum_decls, list):
        enum_decls = []

    return BuilderInput(
        user_box_decls=user_box_decls,
        enum_decls=enum_decls,
        functions=functions,
        call_arities=call_arities,
    )

class MIRReader:
    """MIR JSON reader wrapper - supports v0 and v1 schema"""
    def __init__(self, mir_json: Dict[str, Any]):
        self.mir_json = mir_json
        self.functions = None
        self.schema_version = self._detect_schema_version()
        self.capabilities = self._extract_capabilities()

    def _detect_schema_version(self) -> str:
        """Detect JSON schema version (v0 or v1)"""
        return self.mir_json.get("schema_version", "0.0")

    def _extract_capabilities(self) -> List[str]:
        """Extract capabilities from v1 schema"""
        if self.schema_version.startswith("1."):
            return self.mir_json.get("capabilities", [])
        return []

    def supports_unified_call(self) -> bool:
        """Check if JSON supports unified mir_call instructions"""
        return "unified_call" in self.capabilities
        
    def get_functions(self) -> List[Dict[str, Any]]:
        """Get functions in the expected format for llvm_builder - supports v0/v1 schema"""
        if self.functions is not None:
            return self.functions

        self.functions = normalize_functions_payload(self.mir_json)
        return self.functions

    def get_metadata(self) -> Dict[str, Any]:
        """Get v1 schema metadata (empty dict for v0)"""
        if self.schema_version.startswith("1."):
            return self.mir_json.get("metadata", {})
        return {}
