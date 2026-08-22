      // Legacy-only optional C API probe. Contract-bound modules never enter
      // this include and therefore cannot consult HAKO_CAPI_TM or create a
      // second TargetMachine.
#if !defined(_WIN32)
      {
        const char* tm = getenv("HAKO_CAPI_TM");
        if (tm && tm[0]=='1') {
          typedef void* (*p_LLVMContextCreate)(void);
          typedef void  (*p_LLVMContextDispose)(void*);
          typedef void* (*p_LLVMCreateMemoryBufferWithContentsOfFile)(const char*, char**);
          typedef void  (*p_LLVMDisposeMemoryBuffer)(void*);
          typedef int   (*p_LLVMParseIRInContext)(void*, void*, void**, char**);
          typedef char* (*p_LLVMGetDefaultTargetTriple)(void);
          typedef void  (*p_LLVMDisposeMessage)(char*);
          typedef int   (*p_LLVMGetTargetFromTriple)(const char*, void**, char**);
          typedef void* (*p_LLVMCreateTargetMachine)(void*, const char*, const char*, const char*, int, int, int);
          typedef int   (*p_LLVMTargetMachineEmitToFile)(void*, void*, char*, int, char**);
          typedef void  (*p_LLVMDisposeTargetMachine)(void*);
          typedef void  (*p_LLVMDisposeModule)(void*);
          typedef void* (*p_LLVMModuleCreateWithNameInContext)(const char*, void*);
          typedef void (*p_LLVMInitializeX86TargetInfo)(void);
          typedef void (*p_LLVMInitializeX86Target)(void);
          typedef void (*p_LLVMInitializeX86TargetMC)(void);
          typedef void (*p_LLVMInitializeX86AsmPrinter)(void);

          const char* cand[] = { "libLLVM-18.so", "libLLVM.so.18", "libLLVM.so", NULL };
          void* h = NULL;
          for (int i=0;cand[i];i++) {
            h = dlopen(cand[i], RTLD_LAZY|RTLD_LOCAL);
            if (h) break;
          }
          if (h) {
            p_LLVMContextCreate f_LLVMContextCreate =
                (p_LLVMContextCreate)dlsym(h, "LLVMContextCreate");
            p_LLVMContextDispose f_LLVMContextDispose =
                (p_LLVMContextDispose)dlsym(h, "LLVMContextDispose");
            p_LLVMCreateMemoryBufferWithContentsOfFile f_LLVMCreateMemoryBufferWithContentsOfFile =
                (p_LLVMCreateMemoryBufferWithContentsOfFile)dlsym(
                    h, "LLVMCreateMemoryBufferWithContentsOfFile");
            p_LLVMDisposeMemoryBuffer f_LLVMDisposeMemoryBuffer =
                (p_LLVMDisposeMemoryBuffer)dlsym(h, "LLVMDisposeMemoryBuffer");
            p_LLVMParseIRInContext f_LLVMParseIRInContext =
                (p_LLVMParseIRInContext)dlsym(h, "LLVMParseIRInContext");
            p_LLVMGetDefaultTargetTriple f_LLVMGetDefaultTargetTriple =
                (p_LLVMGetDefaultTargetTriple)dlsym(h, "LLVMGetDefaultTargetTriple");
            p_LLVMDisposeMessage f_LLVMDisposeMessage =
                (p_LLVMDisposeMessage)dlsym(h, "LLVMDisposeMessage");
            p_LLVMGetTargetFromTriple f_LLVMGetTargetFromTriple =
                (p_LLVMGetTargetFromTriple)dlsym(h, "LLVMGetTargetFromTriple");
            p_LLVMCreateTargetMachine f_LLVMCreateTargetMachine =
                (p_LLVMCreateTargetMachine)dlsym(h, "LLVMCreateTargetMachine");
            p_LLVMTargetMachineEmitToFile f_LLVMTargetMachineEmitToFile =
                (p_LLVMTargetMachineEmitToFile)dlsym(h, "LLVMTargetMachineEmitToFile");
            p_LLVMDisposeTargetMachine f_LLVMDisposeTargetMachine =
                (p_LLVMDisposeTargetMachine)dlsym(h, "LLVMDisposeTargetMachine");
            p_LLVMDisposeModule f_LLVMDisposeModule =
                (p_LLVMDisposeModule)dlsym(h, "LLVMDisposeModule");
            p_LLVMModuleCreateWithNameInContext f_LLVMModuleCreateWithNameInContext =
                (p_LLVMModuleCreateWithNameInContext)dlsym(
                    h, "LLVMModuleCreateWithNameInContext");
            p_LLVMInitializeX86TargetInfo f_LLVMInitializeX86TargetInfo =
                (p_LLVMInitializeX86TargetInfo)dlsym(h, "LLVMInitializeX86TargetInfo");
            p_LLVMInitializeX86Target f_LLVMInitializeX86Target =
                (p_LLVMInitializeX86Target)dlsym(h, "LLVMInitializeX86Target");
            p_LLVMInitializeX86TargetMC f_LLVMInitializeX86TargetMC =
                (p_LLVMInitializeX86TargetMC)dlsym(h, "LLVMInitializeX86TargetMC");
            p_LLVMInitializeX86AsmPrinter f_LLVMInitializeX86AsmPrinter =
                (p_LLVMInitializeX86AsmPrinter)dlsym(h, "LLVMInitializeX86AsmPrinter");

            int ok = f_LLVMContextCreate && f_LLVMCreateMemoryBufferWithContentsOfFile &&
                     f_LLVMParseIRInContext && f_LLVMGetDefaultTargetTriple &&
                     f_LLVMGetTargetFromTriple && f_LLVMCreateTargetMachine &&
                     f_LLVMTargetMachineEmitToFile && f_LLVMDisposeTargetMachine &&
                     f_LLVMDisposeMessage && f_LLVMDisposeMemoryBuffer &&
                     f_LLVMContextDispose && f_LLVMDisposeModule &&
                     f_LLVMInitializeX86TargetInfo && f_LLVMInitializeX86Target &&
                     f_LLVMInitializeX86TargetMC && f_LLVMInitializeX86AsmPrinter;
            if (ok) {
              f_LLVMInitializeX86TargetInfo();
              f_LLVMInitializeX86Target();
              f_LLVMInitializeX86TargetMC();
              f_LLVMInitializeX86AsmPrinter();
              void* ctx = f_LLVMContextCreate();
              char* emsg = NULL;
              void* buf = f_LLVMCreateMemoryBufferWithContentsOfFile(llpath, &emsg);
              if (!buf) {
                if (emsg) f_LLVMDisposeMessage(emsg);
                goto TM_END;
              }
              void* mod = NULL;
              if (f_LLVMParseIRInContext(ctx, buf, &mod, &emsg)) {
                if (emsg) f_LLVMDisposeMessage(emsg);
                goto TM_CTX;
              }
              buf = NULL;
              char* triple = f_LLVMGetDefaultTargetTriple();
              void* tgt = NULL;
              if (f_LLVMGetTargetFromTriple(triple, &tgt, &emsg)) {
                if (emsg) f_LLVMDisposeMessage(emsg);
                goto TM_MOD;
              }
              const char* ol = getenv("HAKO_LLVM_OPT_LEVEL");
              if (!ol) ol = getenv("NYASH_LLVM_OPT_LEVEL");
              if (!ol) ol = "0";
              int opt = (ol[0]=='3')?3:(ol[0]=='2')?2:(ol[0]=='1')?1:0;
              void* tmachine = f_LLVMCreateTargetMachine(
                  tgt, triple, "", "", opt, /*Reloc*/0, /*CodeModel*/0);
              if (!tmachine) goto TM_MOD;
              if (f_LLVMTargetMachineEmitToFile(
                      tmachine, mod, (char*)obj_out, /*Object*/1, &emsg)) {
                if (emsg) f_LLVMDisposeMessage(emsg);
              } else {
                rc = 0;
              }
              f_LLVMDisposeTargetMachine(tmachine);
            TM_MOD:
              if (mod) f_LLVMDisposeModule(mod);
              if (triple) f_LLVMDisposeMessage(triple);
              if (buf) f_LLVMDisposeMemoryBuffer(buf);
            TM_CTX:
              f_LLVMContextDispose(ctx);
            TM_END: ;
            }
            dlclose(h);
          }
        }
      }
#endif
