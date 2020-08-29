%%%-------------------------------------------------------------------
%% @doc simple-rest public API
%% @end
%%%-------------------------------------------------------------------

-module(simple-rest_app).

-behaviour(application).

-export([start/2, stop/1]).

start(_StartType, _StartArgs) ->
    Dispatch = cowboy_router:compile([
        {'_', [
            {"/", hello_handler, []}
            ]}
        ]),
        {ok, _} = cowboy:start_clear(my_http_listener,
            [{port, 8080}],
            #{env => #{dispatch => Dispatch},
                    middlewares => [cowboy_router, session_cowboy_middleware, cowboy_handler]
    }),
    simple-rest_sup:start_link().


stop(_State) ->
    ok.

%% internal functions
