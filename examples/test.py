from ..src.cgen import func_def, func_call, return_expr, FuncParam, cendline, endline, write_file, add

# a simple plus 1 function
test_f = func_def("int", "add1",  return_expr(add(1,"val")), [FuncParam("val", "int")])
# body of main function
main_body = func_call("add1", [1]) + cendline() + return_expr("1")
# main func expander
main_func = func_def("int", "main", main_body)
# the entire body of code being written to file
body = test_f + endline() + main_func
# body of code being written to file
# check test.c for the generated code
write_file("test.c", body)