#include <stdio.h>
#include <iostream>
#include <vector>
#include <algorithm>
#include <functional>
#include <queue>

using namespace std;

void kthSmallestElement(vector<int> array, int k) {

    priority_queue<int, vector<int>, greater<int> > pq;
    for (int el : array) {
        pq.push(el);
    }

    for (int i = 1; i < k; ++i) {
        pq.pop();
    }

    cout << pq.top() << endl;
}

int main() {
    int numTests;
    cin >> numTests;

    int size, k;
    for (int i = 0; i < numTests; ++i) {
        cin >> size;

        vector<int> array;
        int number;
        for (int i = 0; i < size; ++i) {
            cin >> number;
            array.push_back(number);
        }

        cin >> k;
        kthSmallestElement(array, k);
    }
}
