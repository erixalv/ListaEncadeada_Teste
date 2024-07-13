#ifndef LISTAENCADEADA_H
#define LISTAENCADEADA_H

#pragma once

#include "Node.h"

class ListaEncadeada
{
public:
    Node* head;
    int size;

    ListaEncadeada();

    bool isEmpty();
    void insertFirst(int d);
    void remove(int d);
    bool search(int d);
};

#endif
