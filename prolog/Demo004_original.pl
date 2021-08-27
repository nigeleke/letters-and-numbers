subsets2(Xs, Xs).
subsets2(Xs, Ys) :- select(_, Xs, Ys).
subsets2(Xs, Ys) :- select(_, Xs, Y1s), subsets2(Y1s, Ys).

subsets(Xs, Xss) :-
    findall(Ys, subsets2(Xs, Ys), Yss),
    list_to_set(Yss, Xss).

operation(O1, O2, V, ' + ') :- V is O1 + O2.
operation(O1, O2, V, ' - ') :- V is O1 - O2, V > 0.
operation(O1, O2, V, ' * ') :- not(O1 = 1), not(O2 = 1), V is O1 * O2.
operation(O1, O2, V, ' / ') :- not(O2 = 1), M is O1 mod O2, M = 0, V is O1 // O2.

findTree([], _, _) :- !, fail.
findTree([X|[]], value(X), X) :- !.
findTree(Xs, tree(O, TL, TR), V) :-
    append(L, R, Xs),
    not(compare(=, L, Xs)), findTree(L, TL, V1),
    not(compare(=, R, Xs)), findTree(R, TR, V2),
    operation(V1, V2, V, O).

makeExpr(L, O, R, E) :-
    concat('(', L, E1),
    concat(E1, O, E2),
    concat(E2, R, E3),
    concat(E3, ')', E).

treeExpr(value(X), X).
treeExpr(tree(O, L, R), E) :-
    treeExpr(L, Le),
    treeExpr(R, Re),
    makeExpr(Le, O, Re, E).

delta_length('<', Xs, Ys) :-
    length(Xs, XL),
    length(Ys, YL),
    compare(<, XL, YL), !.
delta_length('>', _, _).

lettersAndNumbers(V1, V2, V3, V4, V5, V6, Goal, E) :-
    subsets([V1, V2, V3, V4, V5, V6], Xss),
    predsort(delta_length, Xss, Yss),
    member(Ys, Yss),
    permutation(Ys, Y1s),
    findTree(Y1s, T, Goal), !,
    treeExpr(T, E).
lettersAndNumbers(_, _, _, _, _, _, _, 'It''s impossible').
