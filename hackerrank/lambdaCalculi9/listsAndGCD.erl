-module(listsAndGCD).
-export([main/0]).

-record(p, {b :: integer(), e :: integer()}).

main() ->
    {ok, [N]} = io:fread("", "~d"),
    NumberList = read_numbers(N, []),
    print_result(glist(NumberList)).

read_numbers(0, Result) -> Result;
read_numbers(N, Result) ->
    Line = io:get_line(""),
    Number = to_record(split_number(Line)),
    read_numbers(N-1, [Number | Result]).

split_number(Line) -> split_number_int(string:to_integer(Line)).

split_number_int({Int, [32 | Rest]}) -> [Int | split_number(Rest)];
split_number_int({Int, _})           -> [Int];
split_number_int(_)                  -> [].

to_record([]) -> [];
to_record([B, E | Rest]) -> [#p{b = B, e = E} | to_record(Rest)].

print_result([#p{b = B, e = E}]) ->
    io:format("~p ~p~n", [B, E]);
print_result([#p{b = B, e = E} | Rest]) ->
    io:format("~p ~p ", [B, E]),
    print_result(Rest).

glist([A]) -> A;
glist([A | Rest]) -> gdc(A, glist(Rest)).

gdc([], _) -> [];
gdc(_, []) -> [];
gdc([#p{b = B1} | _] = A, [#p{b = B2} | Rest]) when B1 > B2 -> gdc(A, Rest);
gdc([#p{b = B1} | Rest], [#p{b = B2} | _] = B) when B1 < B2 -> gdc(Rest, B);
gdc([#p{e = E1} | RestA], [#p{e = E2} = Result | RestB]) when E1 > E2 -> [Result | gdc(RestA, RestB)];
gdc([Result | RestA], [_ | RestB]) -> [Result | gdc(RestA, RestB)].
