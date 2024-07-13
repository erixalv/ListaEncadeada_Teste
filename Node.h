#ifndef NODE_H
#define NODE_H

#pragma once

class Node
{
public:
    Node(int d, Node* n);
    Node();

    int data;
    Node* next;

};

#endif