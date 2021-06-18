entity Test {
    field : TestType;
    field2    : TestType2;
}

service TestService {
    type test : Test;

    entct1 {
    field3 : Test3;
    }

    action atest(arg1: Test);
    actrns Test;
    action atest2(arg1: Test) returns array of Test;

    function ftest0() returns Test;
    function ftest1(arg1: Test) returns Test;
    function ftest2();
}