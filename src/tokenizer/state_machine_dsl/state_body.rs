macro_rules! state_body {
    ( | [ $self:tt, $input_chunk:ident, $ch:ident ] |> [$($arms:tt)+], [$($enter_actions:tt)*] ) => {
        action_list!(@state_enter |$self, $input_chunk, $ch|> $($enter_actions)*);
        state_body!(@map_arms | [$self, $input_chunk, $ch] |> [$($arms)+], [])
    };


    // Recursively expand each arm's pattern
    //--------------------------------------------------------------------
    ( @map_arms
        | $scope_vars:tt |>
        [ $pat:tt => ( $($actions:tt)* ) $($rest:tt)* ], [ $($expanded:tt)* ]
    ) => {
        arm_pattern!(|[ $scope_vars, [$($rest)*], [$($expanded)*] ]|> $pat => ( $($actions)* ))
    };

    ( @map_arms
        | $scope_vars:tt |>
        [], [$($expanded:tt)*]
    ) => {
        state_body!(@match_block |$scope_vars|> $($expanded)*);
    };


    // Callback for the expand_arm_pattern
    //--------------------------------------------------------------------
    ( @callback
        | [ $scope_vars:tt, [$($pending:tt)*], [$($expanded:tt)*] ] |>
        $($expanded_arm:tt)*
    ) => {
        state_body!(@map_arms | $scope_vars |> [$($pending)*], [$($expanded)* $($expanded_arm)*])
    };


    // Character match block
    //--------------------------------------------------------------------
    ( @match_block
        | [ $self:tt, $input_chunk:ident, $ch:ident ] |>
        $( $pat:pat $(|$pat_cont:pat)* $(if $pat_expr:expr)* => ( $($actions:tt)* ) )*
    ) => {
        match $ch {
            $(
                $pat $(| $pat_cont)* $(if $pat_expr)* => {
                    action_list!(|$self, $input_chunk, $ch|> $($actions)*);
                }
            )*
        }
    };
}
