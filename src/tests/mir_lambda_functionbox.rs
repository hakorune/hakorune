use crate::backend::VM;
use crate::mir::MirCompiler;
use crate::parser::NyashParser;

#[test]
fn lambda_value_then_call_returns_increment() {
    // Nyash code:
    // f = function(a) { return a + 1 }
    // f(41)
    let code = r#"
    f = function(a) {
        return a + 1
    }
    f(41)
    "#;
    let ast = NyashParser::parse_from_string(code).expect("parse");
    let mut mc = MirCompiler::new();
    let cr = mc.compile(ast).expect("mir");
    // Execute on VM
    let mut vm = VM::new();
    let out = vm.execute_module(&cr.module).expect("vm exec");
    // execute_module returns Box<dyn NyashBox>, so downcast to IntegerBox
    use crate::box_trait::IntegerBox;
    if let Some(ib) = out.as_any().downcast_ref::<IntegerBox>() {
        assert_eq!(ib.value, 42);
    } else {
        panic!("Expected IntegerBox(42), got {:?}", out);
    }
}
