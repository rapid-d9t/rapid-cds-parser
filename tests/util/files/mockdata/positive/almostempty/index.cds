entity Test {
    field : TestType;
}

service TestService {
    action atest(arg1: Test);

    function ftest0() returns Test;
}