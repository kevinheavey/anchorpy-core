from anchorpy_core.idl import Idl, IdlField, IdlAccount, IdlTypeSimple, IdlErrorCode, IdlTypeDefinitionTyStruct

import json
from pathlib import Path


def test_idls() -> None:
    idls = []
    for path in Path("tests/idls/").iterdir():
        raw = path.read_text()
        idl = Idl.from_json(raw)
        idls.append(idl)
    assert idls

def test_clientgen_example() -> None:
    path = Path("tests/idls/clientgen_example_program.json")
    raw = path.read_text()
    idl = Idl.from_json(raw)
    assert idl.version == "0.1.0"
    assert idl.name == "example_program"
    assert idl.docs is None
    assert idl.constants == []
    second_ix = idl.instructions[1]
    assert second_ix.name == "initializeWithValues"
    assert second_ix.docs is None
    assert second_ix.returns is None
    first_arg = second_ix.args[0]
    assert first_arg == IdlField(name="boolField", docs=None, ty=IdlTypeSimple.Bool)
    first_acc_for_ix = second_ix.accounts[0]
    assert first_acc_for_ix == IdlAccount(name="state", is_mut=True, is_signer=True, docs=None, pda=None, relations=[])
    assert idl.state is None
    first_acc = idl.accounts[0]
    assert first_acc.name == "State"
    assert first_acc.docs is None
    first_acc_ty = first_acc.ty
    assert first_acc_ty
    assert isinstance(first_acc_ty, IdlTypeDefinitionTyStruct)
    assert first_acc_ty.fields[0] == IdlField(name="boolField", docs=None, ty=IdlTypeSimple.Bool)
    first_type = idl.types[0]
    assert first_type.name == "BarStruct"
    assert first_type.docs is None
    first_type_ty = first_type.ty
    assert first_type_ty.fields[0] == IdlField(name="someField", docs=None, ty=IdlTypeSimple.Bool)
    assert idl.events is None
    assert idl.errors[0] == IdlErrorCode(code=6000, name="SomeError", msg="Example error.")
    assert idl.metadata is None
