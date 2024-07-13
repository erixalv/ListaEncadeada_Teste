#include "ListaEncadeada.h"

ListaEncadeada::ListaEncadeada() {
    head = nullptr;
    size = 0;
}

bool ListaEncadeada::isEmpty() {
    return head == nullptr;
}

void ListaEncadeada::insertFirst(int d) {
    if(isEmpty()) {
        head = new Node(d, nullptr);
    } 
    else {
        Node* newNode = new Node(d, head);
        head = newNode;
    }
    size++;
}

bool ListaEncadeada::search(int d) {
    if (!isEmpty()) {
        Node* auxNode = head;
        while (auxNode != nullptr && auxNode->data != d) {
            auxNode = auxNode->next;
        }
        return auxNode->data == d;
    }
    else {
        return false;
    }
}

void ListaEncadeada::remove(int d) {
    if (!isEmpty()) {
        if (head->data == d) {
        Node* toDelete = head;
        head = head->next;
        delete toDelete;
        size--;
        return;
    }

        Node *auxNode = head->next;
        Node *prevAuxNode = head;
        while (auxNode != nullptr && auxNode->data != d) {
            auxNode = auxNode->next;
            prevAuxNode = prevAuxNode->next;
        }
        if (auxNode->data == d) {
            prevAuxNode->next = auxNode->next;
        }
        if (auxNode != nullptr) {
        prevAuxNode->next = auxNode->next;
        delete auxNode;
        size--;
    }
    }
}