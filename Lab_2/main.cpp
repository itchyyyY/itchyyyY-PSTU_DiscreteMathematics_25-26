#include <iostream>
#include <vector>

using namespace std;

const int N = 6;

bool isReflexive(const vector<vector<int>>& mat) {
    for (int i = 0; i < N; i++)
        if (mat[i][i] != 1)
            return false;
    return true;
}

bool isIrreflexive(const vector<vector<int>>& mat) {
    for (int i = 0; i < N; i++)
        if (mat[i][i] != 0)
            return false;
    return true;
}

bool isSymmetric(const vector<vector<int>>& mat) {
    for (int i = 0; i < N; i++)
        for (int j = 0; j < N; j++)
            if (i != j && mat[i][j] != mat[j][i])
                return false;
    return true;
}

bool isAntisymmetric(const vector<vector<int>>& mat) {
    for (int i = 0; i < N; i++)
        for (int j = 0; j < N; j++)
            if (i != j && mat[i][j] == 1 && mat[j][i] == 1)
                return false;
    return true;
}

bool isAsymmetric(const vector<vector<int>>& mat) {
    for (int i = 0; i < N; i++)
        for (int j = 0; j < N; j++)
            if (mat[i][j] == 1 && mat[j][i] == 1)
                return false;
    return true;
}

bool isTransitive(const vector<vector<int>>& mat) {
    for (int i = 0; i < N; i++)
        for (int j = 0; j < N; j++)
            for (int k = 0; k < N; k++)
                if (mat[i][j] && mat[j][k] && !mat[i][k])
                    return false;
    return true;
}

bool isConnected(const vector<vector<int>>& mat) {
    for (int i = 0; i < N; i++)
        for (int j = 0; j < N; j++)
            if (i != j && mat[i][j] == 0 && mat[j][i] == 0)
                return false;
    return true;
}

int main() {
    setlocale(LC_ALL, "Ru");

    vector<vector<int>> mat(N, vector<int>(N));

    // mat1
    /*mat = {
        {1, 0, 1, 0, 0, 1},
        {0, 1, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 1},
        {0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0}
    };*/
    
    // mat2
    /*mat = {
        {1, 0, 1, 0, 0, 1},
        {0, 1, 0, 0, 0, 0},
        {1, 0, 1, 0, 0, 1},
        {0, 0, 0, 1, 0, 0},
        {0, 0, 0, 0, 1, 0},
        {1, 0, 1, 0, 0, 1}
    };*/

    // mat3
    /*mat = {
        {0, 0, 1, 0, 0, 1},
        {1, 0, 0, 0, 0, 0},
        {0, 1, 0, 0, 0, 1},
        {1, 1, 1, 0, 0, 0},
        {1, 1, 1, 1, 0, 0},
        {0, 1, 0, 1, 1, 0}
    };*/

    // mat4
    /*mat = {
        {0, 1, 1, 1, 1, 0},
        {0, 1, 1, 1, 1, 0},
        {0, 1, 1, 1, 1, 0},
        {0, 1, 1, 1, 1, 0},
        {0, 1, 1, 1, 1, 0},
        {0, 1, 1, 1, 1, 0}
    };*/

    // mat5
    /*mat = {
        {0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0},
        {1, 1, 1, 1, 1, 1},
        {1, 1, 1, 1, 1, 1},
        {0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0}
    };*/

    // mat7
    /*mat = {
        {1, 0, 1, 1, 1, 0},
        {1, 1, 1, 1, 1, 1},
        {0, 0, 1, 1, 1, 1},
        {0, 0, 0, 1, 1, 1},
        {0, 0, 0, 0, 1, 1},
        {1, 0, 0, 0, 0, 1}
    };*/

    // mat8
    /*mat = {
        {0, 1, 1, 1, 1, 1},
        {1, 0, 1, 1, 1, 1},
        {1, 1, 0, 1, 1, 1},
        {1, 1, 1, 0, 1, 1},
        {1, 1, 1, 1, 0, 1},
        {1, 1, 1, 1, 1, 0}
    };*/

    bool reflexive = isReflexive(mat);
    bool irreflexive = isIrreflexive(mat);
    bool symmetric = isSymmetric(mat);
    bool antisymmetric = isAntisymmetric(mat);
    bool asymmetric = isAsymmetric(mat);
    bool transitive = isTransitive(mat);
    bool connected = isConnected(mat);

    cout << "Свойства отношения:\n";
    cout << "1. Рефлексивность: " << (reflexive ? "да" : "нет") << "\n";
    cout << "2. Антирефлексивность: " << (irreflexive ? "да" : "нет") << "\n";
    cout << "3. Симметричность: " << (symmetric ? "да" : "нет") << "\n";
    cout << "4. Антисимметричность: " << (antisymmetric ? "да" : "нет") << "\n";
    cout << "5. Ассиметричность: " << (asymmetric ? "да" : "нет") << "\n";
    cout << "6. Транзитивность: " << (transitive ? "да" : "нет") << "\n";
    cout << "7. Связность: " << (connected ? "да" : "нет") << "\n";

    if (reflexive && symmetric && transitive)
        cout << "Отношение является отношением эквивалентности.\n";

    if (reflexive && antisymmetric && transitive)
        cout << "Отношение является отношением порядка.\n";

    return 0;
}