service TestService {
    action atest1(arg1: Test);
    action atest(arg1: Test, arg2: Test);

    function ftest1() returns Test;
    function ftest2(arg1: Test) returns Test;
    function ftest3(arg1: Test, arg2: Test2) returns Test;
    function ftest4(arg1: Test, arg2: Test2, arg3: Test3) returns Test;
}