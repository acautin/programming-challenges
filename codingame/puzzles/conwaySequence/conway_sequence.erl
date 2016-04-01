-module(conway_sequence).

-export([get_line/2]).

-spec get_line(integer(), integer()) -> binary().
get_line(_, LineNumber) when LineNumber < 1 ->
    {error, <<"Line number must be bigger than or equal to 1">>};
get_line(Seed, LineNumber) ->
    %% LineNumber-1 as the seed is the first one.
    Line = iterate_next(LineNumber - 1, [Seed]),
    list_to_binary(string:join([integer_to_list(E) || E <- Line], " ")).

iterate_next(0, Result) -> Result;
iterate_next(N, Acc) ->
    iterate_next(N-1, next(Acc)).

next([]) -> [];
next([X | Rest]) ->
    {H, T} = lists:splitwith(fun(E) -> E =:= X end, Rest),
    [length(H) + 1, X | next(T)].
