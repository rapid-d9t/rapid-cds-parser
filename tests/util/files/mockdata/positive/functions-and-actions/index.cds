service TestService {
    action atest1(arg1: Test);
    action atest(arg1: Test, arg2: Test);

    function ftest0() returns Test;
    function ftest0(arg1: Test) returns Test;
    function ftest0(arg1: Test, arg2: Test2) returns Test;
}