lettersAndNumbers(V1, V2, V3, V4, V5, V6, Goal, Expression) :-
    permutationOfCombination([V1, V2, V3, V4, V5, V6], Vs),
    listTree(Vs, T),
    value(T, V),
    V = Goal,
    treeExpression(T, Expression).
lettersAndNumbers(_, _, _, _, _, _, _, 'It''s impossible').

permutationOfCombination(Xs, Ys) :- combination(Xs, X1s), permutation(X1s, Ys), not(length(Ys, 0)).

combination([],[]).
combination([H|T], [H|T2]) :- combination(T,T2).
combination([_|T], T2) :- combination(T,T2).

listTree(Xs, leaf(X)) :- member(X, Xs).
listTree(Xs, node(Op, Lt, Rt)) :-
    appendNonEmpty(Ls, Rs, Xs),
    listTree(Ls, Lt),
    listTree(Rs, Rt),
    operation(Op).

appendNonEmpty(Ls, Rs, Xs) :-
    append(Ls, Rs, Xs),
    not(length(Ls, 0)),
    not(length(Rs, 0)).

operation(+).
operation(-).
operation(*).
operation(//).

value(leaf(X), X).
value(node(Op, Lt, Rt), V) :-
    value(Lt, L),
    value(Rt, R),
    operation(Op, L, R, V).

operation(+, L, R, V) :- V is L + R.
operation(-, L, R, V) :- V is L - R, V > 0.
operation(*, L, R, V) :- not(L = 1), not(R = 1), V is L * R.
operation(//, L, R, V) :- not(R = 0), not(R = 1), M is L mod R, M is 0, V is L // R.

treeExpression(leaf(X), X).
treeExpression(node(Op, Lt, Rt), S) :-
    treeExpression(Lt, Le),
    treeExpression(Rt, Re),
    atomics_to_string(['(', Le, Op, Re, ')'], S).
