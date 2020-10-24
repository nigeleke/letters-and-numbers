person(alf, male).
person(beatrice, female).
person(clarie, male).
person(dot, female).
person(ernie, male).
person(frida, female).
person(adam, male).
person(denise, female).
person(chris, male).
person(faith, female).
person(darlene, female).
person(dawn, female).
person(clint, male).
person(chuck, male).

parent(adam, alf).
parent(adam, beatrice).
parent(denise, clarie).
parent(denise, dot).
parent(chris, clarie).
parent(chris, dot).
parent(faith, ernie).
parent(faith, frida).
parent(darlene, adam).
parent(darlene, denise).
parent(dawn, adam).
parent(dawn, denise).
parent(clint, chris).
parent(clint, faith).
parent(chuck, chris).
parent(chuck, faith).

father(Child, Parent) :- parent(Child, Parent), person(Parent, male).
mother(Child, Parent) :- parent(Child, Parent), person(Parent, female), !.

% father(C, F)
% father(clint, F) <- false.
% mother(C,M).
% mother(client, M).
% mother(frida, M).
