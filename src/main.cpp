#include <iostream>
#include <string>
#include <vector>
using namespace std;
class Solution {
public:
    static vector<string> generateParenthesis(int n) {
        vector<vector<string>> m = {{""s}};
        for (auto i = 1; i <= n; i++) {
            vector<string> v;
            for (auto j = 0; j < i; j++) {
                for (const auto p : m[j]) {
                    for (const auto q : m[i - 1 - j]) {
                        v.emplace_back(p + '(' + q + ')');
                    }
                }
            }
            m.emplace_back(move(v));
        }
        return m.back();
    }
};

void generate_parenthesis(int n, int ln, int rn, string &s, vector<string> &res)
{
    if (ln + rn >= n + n) {
        res.push_back(s);
        return;
    }

    if (ln < n) {
        s.push_back('(');
        generate_parenthesis(n, ln + 1, rn, s, res);
        s.pop_back();
    }

    if (rn < n && ln > rn) {
        s.push_back(')');
        generate_parenthesis(n, ln, rn + 1, s, res);
        s.pop_back();
    }

}

vector<string> generate_parenthesis(int n)
{
    vector<string> res;
    string s;
    generate_parenthesis(n, 0, 0, s, res);
    return res;
}

int main()
{
    for (auto i : Solution::generateParenthesis(3)) {
        cout << i << endl;
    }
    return 0;
}