-module(archery).
-export([main/0]).

-record(arrow, {
          p1 :: float(),
          p2 :: float()
         }).

main() ->
    {ok, [N]} = io:fread("", "~d"),
    RadiusList = lists:sort(read_radius(N, [])),
    {ok, [M]} = io:fread("", "~d"),
    ArrowsList = read_arrows(M, []),
    io:format("~p~n", [count_q(RadiusList, ArrowsList)]).

read_radius(0, Result) -> Result;
read_radius(N, Result) ->
    {ok, [Radius]} = io:fread("", "~d"),
    read_radius(N-1, [Radius | Result]).

read_arrows(0, Result) -> Result;
read_arrows(M, Result) ->
    {ok, [X1, Y1, X2, Y2]} = io:fread("", "~d~d~d~d"),
    D1 = math:sqrt(math:pow(X1, 2) + math:pow(Y1, 2)),
    D2 = math:sqrt(math:pow(X2, 2) + math:pow(Y2, 2)),
    if
        D1 < D2 ->
            P1 = D1,
            P2 = D2;
        true ->
            P2 = D1,
            P1 = D2
    end,
    read_arrows(M-1, [#arrow{p1 = P1, p2 = P2} | Result]).

count_q(RadiusList, ArrowsList) ->
    count_q(RadiusList, ArrowsList, 0).

count_q(_, [], Result) -> Result;
count_q(RadiusList, [Arrow | Rest], Result) ->
    count_q(RadiusList, Rest, Result + count_intersect(RadiusList, Arrow)).

count_intersect(RadiusList, Arrow) ->
    count_intersect(RadiusList, Arrow, 0).

count_intersect([], _, Result) -> Result;
count_intersect([Radius | _], #arrow{p2 = P2}, Result) when Radius > P2 -> Result;
count_intersect([Radius | Rest], #arrow{p1 = P1} = Arrow, Result) when Radius > P1 ->
    count_intersect(Rest, Arrow, Result + 1);
count_intersect([_ | Rest], Arrow, Result) ->
    count_intersect(Rest, Arrow, Result).
