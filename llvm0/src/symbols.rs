use llvm_sys::analysis::*;
use llvm_sys::core::*;
use llvm_sys::error::*;
use llvm_sys::execution_engine::*;
use llvm_sys::support::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::pass_manager_builder::*;
use std::ffi::CStr;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(C)]
pub struct Symbol {
    pub name: *const i8,
    pub addr: *const u8,
}

unsafe impl Send for Symbol {}
unsafe impl Sync for Symbol {}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(C)]
pub struct SymbolArray {
    pub ptr: *const Symbol,
    pub len: u64,
}

macro_rules! symbol {
    ($name:ident) => {
        Symbol {
            name: unsafe {
                CStr::from_bytes_with_nul_unchecked(concat!(stringify!($name), "\0").as_bytes())
                    .as_ptr()
            },
            addr: $name as *const u8,
        }
    };
}

static SYMBOLS: &[Symbol] = &[
    symbol!(llvm_symbols),
    symbol!(LLVMABIAlignmentOfType),
    symbol!(LLVMABISizeOfType),
    symbol!(LLVMAddFunction),
    symbol!(LLVMAddGlobal),
    symbol!(LLVMAddGlobalInAddressSpace),
    symbol!(LLVMAddIncoming),
    symbol!(LLVMAddModule),
    symbol!(LLVMAddSymbol),
    symbol!(LLVMAlignOf),
    symbol!(LLVMAppendBasicBlockInContext),
    symbol!(LLVMArrayType),
    symbol!(LLVMBuildAShr),
    symbol!(LLVMBuildAdd),
    symbol!(LLVMBuildAddrSpaceCast),
    symbol!(LLVMBuildAlloca),
    symbol!(LLVMBuildAnd),
    symbol!(LLVMBuildArrayAlloca),
    symbol!(LLVMBuildBitCast),
    symbol!(LLVMBuildBr),
    symbol!(LLVMBuildCall),
    symbol!(LLVMBuildCondBr),
    symbol!(LLVMBuildExtractValue),
    symbol!(LLVMBuildFAdd),
    symbol!(LLVMBuildFCmp),
    symbol!(LLVMBuildFDiv),
    symbol!(LLVMBuildFMul),
    symbol!(LLVMBuildFNeg),
    symbol!(LLVMBuildFPExt),
    symbol!(LLVMBuildFPToSI),
    symbol!(LLVMBuildFPToUI),
    symbol!(LLVMBuildFPTrunc),
    symbol!(LLVMBuildFRem),
    symbol!(LLVMBuildFSub),
    symbol!(LLVMBuildGEP),
    symbol!(LLVMBuildICmp),
    symbol!(LLVMBuildInsertValue),
    symbol!(LLVMBuildIntToPtr),
    symbol!(LLVMBuildLShr),
    symbol!(LLVMBuildLoad),
    symbol!(LLVMBuildMul),
    symbol!(LLVMBuildNSWAdd),
    symbol!(LLVMBuildNSWMul),
    symbol!(LLVMBuildNSWNeg),
    symbol!(LLVMBuildNSWSub),
    symbol!(LLVMBuildNeg),
    symbol!(LLVMBuildNot),
    symbol!(LLVMBuildOr),
    symbol!(LLVMBuildPhi),
    symbol!(LLVMBuildPtrToInt),
    symbol!(LLVMBuildRet),
    symbol!(LLVMBuildRetVoid),
    symbol!(LLVMBuildSDiv),
    symbol!(LLVMBuildSExt),
    symbol!(LLVMBuildSIToFP),
    symbol!(LLVMBuildSRem),
    symbol!(LLVMBuildSelect),
    symbol!(LLVMBuildShl),
    symbol!(LLVMBuildStore),
    symbol!(LLVMBuildStructGEP),
    symbol!(LLVMBuildSub),
    symbol!(LLVMBuildTrunc),
    symbol!(LLVMBuildUDiv),
    symbol!(LLVMBuildUIToFP),
    symbol!(LLVMBuildURem),
    symbol!(LLVMBuildUnreachable),
    symbol!(LLVMBuildXor),
    symbol!(LLVMBuildZExt),
    symbol!(LLVMByteOrder),
    symbol!(LLVMCloneModule),
    symbol!(LLVMConstAllOnes),
    symbol!(LLVMConstArray),
    symbol!(LLVMConstBitCast),
    symbol!(LLVMConstInt),
    symbol!(LLVMConstIntGetSExtValue),
    symbol!(LLVMConstIntGetZExtValue),
    symbol!(LLVMConstIntToPtr),
    symbol!(LLVMConstNamedStruct),
    symbol!(LLVMConstNull),
    symbol!(LLVMConstPtrToInt),
    symbol!(LLVMConstReal),
    symbol!(LLVMConstRealGetDouble),
    symbol!(LLVMConstStringInContext),
    symbol!(LLVMConstStructInContext),
    symbol!(LLVMConstTruncOrBitCast),
    symbol!(LLVMConstVector),
    symbol!(LLVMConsumeError),
    symbol!(LLVMContextCreate),
    symbol!(LLVMContextDispose),
    symbol!(LLVMCopyStringRepOfTargetData),
    symbol!(LLVMCountBasicBlocks),
    symbol!(LLVMCountParamTypes),
    symbol!(LLVMCountParams),
    symbol!(LLVMCountStructElementTypes),
    symbol!(LLVMCreateBuilderInContext),
    symbol!(LLVMCreateFunctionPassManagerForModule),
    symbol!(LLVMCreateInterpreterForModule),
    symbol!(LLVMCreateMCJITCompilerForModule),
    symbol!(LLVMCreateMessage),
    symbol!(LLVMCreatePassManager),
    symbol!(LLVMCreateTargetData),
    symbol!(LLVMCreateTargetDataLayout),
    symbol!(LLVMCreateTargetMachine),
    symbol!(LLVMDeleteBasicBlock),
    symbol!(LLVMDeleteFunction),
    symbol!(LLVMDeleteGlobal),
    symbol!(LLVMDisposeBuilder),
    symbol!(LLVMDisposeErrorMessage),
    symbol!(LLVMDisposeExecutionEngine),
    symbol!(LLVMDisposeMessage),
    symbol!(LLVMDisposeModule),
    symbol!(LLVMDisposePassManager),
    symbol!(LLVMDisposeTargetData),
    symbol!(LLVMDisposeTargetMachine),
    symbol!(LLVMDoubleTypeInContext),
    symbol!(LLVMElementAtOffset),
    symbol!(LLVMFP128TypeInContext),
    symbol!(LLVMFinalizeFunctionPassManager),
    symbol!(LLVMFindFunction),
    symbol!(LLVMFloatTypeInContext),
    symbol!(LLVMFunctionType),
    symbol!(LLVMGetArrayLength),
    symbol!(LLVMGetAsString),
    symbol!(LLVMGetBasicBlockName),
    symbol!(LLVMGetBasicBlockParent),
    symbol!(LLVMGetBasicBlocks),
    symbol!(LLVMGetDefaultTargetTriple),
    symbol!(LLVMGetElementType),
    symbol!(LLVMGetEntryBasicBlock),
    symbol!(LLVMGetErrorMessage),
    symbol!(LLVMGetExecutionEngineTargetData),
    symbol!(LLVMGetExecutionEngineTargetMachine),
    symbol!(LLVMGetFirstFunction),
    symbol!(LLVMGetFirstGlobal),
    symbol!(LLVMGetFirstInstruction),
    symbol!(LLVMGetFirstTarget),
    symbol!(LLVMGetFunctionAddress),
    symbol!(LLVMGetInitializer),
    symbol!(LLVMGetInsertBlock),
    symbol!(LLVMGetIntTypeWidth),
    symbol!(LLVMGetLinkage),
    symbol!(LLVMGetModuleContext),
    symbol!(LLVMGetModuleDataLayout),
    symbol!(LLVMGetNamedFunction),
    symbol!(LLVMGetNamedGlobal),
    symbol!(LLVMGetNextFunction),
    symbol!(LLVMGetNextGlobal),
    symbol!(LLVMGetNextTarget),
    symbol!(LLVMGetParamTypes),
    symbol!(LLVMGetParams),
    symbol!(LLVMGetPointerAddressSpace),
    symbol!(LLVMGetReturnType),
    symbol!(LLVMGetStructElementTypes),
    symbol!(LLVMGetStructName),
    symbol!(LLVMGetTarget),
    symbol!(LLVMGetTargetDescription),
    symbol!(LLVMGetTargetFromName),
    symbol!(LLVMGetTargetFromTriple),
    symbol!(LLVMGetTargetMachineCPU),
    symbol!(LLVMGetTargetMachineFeatureString),
    symbol!(LLVMGetTargetMachineTarget),
    symbol!(LLVMGetTargetMachineTriple),
    symbol!(LLVMGetTargetName),
    symbol!(LLVMGetTypeContext),
    symbol!(LLVMGetTypeKind),
    symbol!(LLVMGetUndef),
    symbol!(LLVMGetValueName2),
    symbol!(LLVMGetVectorSize),
    symbol!(LLVMGetVisibility),
    symbol!(LLVMHalfTypeInContext),
    symbol!(LLVMInitializeFunctionPassManager),
    symbol!(LLVMInitializeMCJITCompilerOptions),
    symbol!(LLVMIntTypeInContext),
    symbol!(LLVMIsAConstantArray),
    symbol!(LLVMIsAConstantDataArray),
    symbol!(LLVMIsAConstantFP),
    symbol!(LLVMIsAConstantInt),
    symbol!(LLVMIsAConstantStruct),
    symbol!(LLVMIsAConstantVector),
    symbol!(LLVMIsAFunction),
    symbol!(LLVMIsAGlobalValue),
    symbol!(LLVMIsAGlobalVariable),
    symbol!(LLVMIsAPHINode),
    symbol!(LLVMIsConstant),
    symbol!(LLVMIsDeclaration),
    symbol!(LLVMIsFunctionVarArg),
    symbol!(LLVMIsGlobalConstant),
    symbol!(LLVMIsLiteralStruct),
    symbol!(LLVMIsNull),
    symbol!(LLVMIsOpaqueStruct),
    symbol!(LLVMIsPackedStruct),
    symbol!(LLVMIsUndef),
    symbol!(LLVMLinkInInterpreter),
    symbol!(LLVMLinkInMCJIT),
    symbol!(LLVMModuleCreateWithNameInContext),
    symbol!(LLVMOffsetOfElement),
    symbol!(LLVMPPCFP128TypeInContext),
    symbol!(LLVMPassManagerBuilderCreate),
    symbol!(LLVMPassManagerBuilderDispose),
    symbol!(LLVMPassManagerBuilderPopulateFunctionPassManager),
    symbol!(LLVMPassManagerBuilderPopulateModulePassManager),
    symbol!(LLVMPassManagerBuilderSetOptLevel),
    symbol!(LLVMPassManagerBuilderSetSizeLevel),
    symbol!(LLVMPointerSize),
    symbol!(LLVMPointerSizeForAS),
    symbol!(LLVMPointerType),
    symbol!(LLVMPositionBuilderAtEnd),
    symbol!(LLVMPositionBuilderBefore),
    symbol!(LLVMPreferredAlignmentOfType),
    symbol!(LLVMPrintModuleToString),
    symbol!(LLVMPrintTypeToString),
    symbol!(LLVMPrintValueToString),
    symbol!(LLVMRemoveModule),
    symbol!(LLVMRunFunctionPassManager),
    symbol!(LLVMRunPassManager),
    symbol!(LLVMSetGlobalConstant),
    symbol!(LLVMSetInitializer),
    symbol!(LLVMSetLinkage),
    symbol!(LLVMSetModuleDataLayout),
    symbol!(LLVMSetTarget),
    symbol!(LLVMSetValueName2),
    symbol!(LLVMSetVisibility),
    symbol!(LLVMSizeOf),
    symbol!(LLVMSizeOfTypeInBits),
    symbol!(LLVMStoreSizeOfType),
    symbol!(LLVMStructCreateNamed),
    symbol!(LLVMStructSetBody),
    symbol!(LLVMStructTypeInContext),
    symbol!(LLVMTargetHasAsmBackend),
    symbol!(LLVMTargetHasJIT),
    symbol!(LLVMTargetHasTargetMachine),
    symbol!(LLVMTargetMachineEmitToFile),
    symbol!(LLVMTypeIsSized),
    symbol!(LLVMTypeOf),
    symbol!(LLVMVectorType),
    symbol!(LLVMVerifyModule),
    symbol!(LLVMVoidTypeInContext),
    symbol!(LLVMX86FP80TypeInContext),
    symbol!(LLVM_InitializeNativeAsmParser),
    symbol!(LLVM_InitializeNativeAsmPrinter),
    symbol!(LLVM_InitializeNativeDisassembler),
    symbol!(LLVM_InitializeNativeTarget),
];

#[no_mangle]
pub extern "C" fn llvm_symbols() -> SymbolArray {
    SymbolArray {
        ptr: &SYMBOLS[0],
        len: SYMBOLS.len() as u64,
    }
}

pub fn get() -> impl IntoIterator<Item = (&'static CStr, *const u8)> {
    SYMBOLS
        .iter()
        .map(|sym| (unsafe { CStr::from_ptr(sym.name) }, sym.addr))
}
