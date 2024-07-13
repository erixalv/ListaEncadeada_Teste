#include "Node.h"
#include "ListaEncadeada.h"
#include <iostream>

using namespace std;

int main() {
    ListaEncadeada linkedList;

    cout << "Lista vazia: " << (linkedList.isEmpty() ? "Sim" : "Não") << endl;

    linkedList.insertFirst(10);
    cout << "Lista vazia: " << (linkedList.isEmpty() ? "Sim" : "Não") << endl;

    cout << "Procurando 10: " << (linkedList.search(10) ? "Encontrado" : "Não Encontrado") << endl;

    linkedList.remove(10);
    cout << "Procurando 10 após remoção: " << (linkedList.search(10) ? "Encontrado" : "Não Encontrado") << endl;

    cout << "Lista vazia: " << (linkedList.isEmpty() ? "Sim" : "Não") << endl;

    return 0;
}
