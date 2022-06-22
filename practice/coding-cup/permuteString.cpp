using namespace std;

void printPermutations(string s, string current) {

    if (s.empty()) {
        cout << current << endl;
    }

    for (int i = 0; i < s.size(); ++i) {
        char letter = s[i];
        s = s.substr(1);
        current += letter;
        printPermutations(s, current);
    }
}

int main() {
    printPermutations("PermuteThisString", "");
}
