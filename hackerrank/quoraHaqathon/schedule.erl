-module(schedule).
-export([main/0]).

-record(test, {
          t :: integer(),  % Time to run the test.
          p :: float(),    % Probability to pass.
          ef :: float(),
          ep :: float()
         }).

main() ->
    {ok, [N]} = io:fread("", "~d"),
    TestList = read_tests(N, []),
    %io:format("~p~n", [TestList]),
    Res = lists:min([calculate_total(sort_tests_ef_le(TestList)),
     calculate_total(sort_tests_ef_g(TestList)),
     calculate_total(sort_tests_ep_le(TestList)),
     calculate_total(sort_tests_ep_g(TestList))]),
    io:format("~p~n", [Res]).

read_tests(0, Result) -> Result;
read_tests(N, Result) ->
    {ok, [T, P]} = io:fread("", "~d~f"),
    EF = T*(1-P),
    EP = T*P,
    read_tests(N-1, [#test{t = T, p = P, ef = EF, ep = EP} | Result]).

calculate_total(TestList) ->
    calculate_total(TestList, 0, 1, 0).

calculate_total([#test{t = Time}], TimeAc, ProbAc, Result) ->
    Expected = (TimeAc + Time)*ProbAc,
    Result + Expected;
calculate_total([#test{t = Time, p = ProbPass} | Rest], TimeAc, ProbAc, Result) ->
    Expected = (TimeAc + Time) * (1 - ProbPass)*ProbAc,
    calculate_total(Rest, TimeAc + Time, ProbPass*ProbAc, Result + Expected).

sort_tests_ep_le(TestList) ->
    SortFun =
        fun(#test{ep = Expected1}, #test{ep = Expected2}) ->
            Expected1 =< Expected2
        end,
    lists:sort(SortFun, TestList).

sort_tests_ep_g(TestList) ->
    SortFun =
        fun(#test{ep = Expected1}, #test{ep = Expected2}) ->
            Expected1 > Expected2
        end,
    lists:sort(SortFun, TestList).

sort_tests_ef_le(TestList) ->
    SortFun =
        fun(#test{ef = Expected1}, #test{ef = Expected2}) ->
            Expected1 =< Expected2
        end,
    lists:sort(SortFun, TestList).

sort_tests_ef_g(TestList) ->
    SortFun =
        fun(#test{ef = Expected1}, #test{ef = Expected2}) ->
            Expected1 > Expected2
        end,
    lists:sort(SortFun, TestList).
