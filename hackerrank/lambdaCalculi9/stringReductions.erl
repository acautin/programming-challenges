-module(stringReductions).
-export([main/0]).

%% Work on this using binary operation to flag.

main() ->
    
    io:format("~s~n", [remove_duplicated(io:get_chars("", 1), TableId, 0)]).

main(IoDevice) ->

    TableId = ets:new(myTable, [set]),
    io:format("~s~n", [remove_duplicated(io:get_chars("", 1), TableId, 0)]).



remove_duplicated(eof, TableId, _) ->
    return_result(TableId);
remove_duplicated(_, TableId, Count) when Count >= 26 ->
    return_result(TableId);
remove_duplicated([C], TableId, Count) ->
    case ets:member(TableId, C) of
        true -> remove_duplicated(io:get_chars("", 1), TableId, Count);
        false ->
            true = ets:insert(TableId, {C, Count}),
            remove_duplicated(io:get_chars("", 1), TableId, Count + 1)
    end.

return_result(TableId) ->
    Sorted = lists:sort(fun({_, A}, {_, B} ) -> A < B end, ets:tab2list(TableId)),
    [C || {C, _} <- Sorted].
