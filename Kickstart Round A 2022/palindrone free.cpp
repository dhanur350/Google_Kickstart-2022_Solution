#include <bits/stdc++.h>
#pragma GCC optimize("O2,unroll-loops")
#pragma GCC target("sse,sse2,sse3,ssse3,sse4,popcnt,abm,mmx,avx,tune=native")
using namespace std;

#include <bits/extc++.h>
using namespace __gnu_pbds;

template <class T>
using ordered_set = tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;

typedef long long ll;
typedef long double ld;
typedef pair<int, int> pi;
typedef pair<ll, ll> pl;
typedef pair<ld, ld> pd;
typedef vector<int> vi;
typedef vector<ll> vl;
typedef vector<ld> vd;
#define FASTIO                        \
    ios_base::sync_with_stdio(false); \
    cin.tie(NULL);                    \
    cout.tie(NULL);
#define PRECISION std::cout << std::fixed << std::setprecision(20);
#define DBPRECISION std::cout << std::fixed << std::setprecision(4);
#define iter(x) (x).begin(), (x).end()
#define umap unordered_map
#define uset unordered_set
#define _f first
#define _s second
const char nl = '\n';
const int INF = (1 << 30) - 1;
const ll LINF = 1e18;
const ld PI = 3.14159265358979323846L;
const ld E = 2.71828182845904523536L;
const ld eps = 1e-6;
#define dbx(x) cout << x << endl;
#define dbn(x) cout << x << nl;
#define dba(x) cout << x << " ";

#define res cout << "Case #" << curr << ": "
const int N = 5e4 + 5;
const int K = (1 << 6) + 5;
bool dp[N][K];
bool bad5[K], bad6[K];
string s;
bool solve(int i, int k5, int k6)
{
    if (bad5[k5] && i >= 5)
    {
        return 0;
    }
    if (bad6[k6] && i >= 6)
    {
        return 0;
    }
    if (i == (int)s.size())
    {
        return 1;
    }
    if (dp[i][k6])
    {
        return 0;
    }
    dp[i][k6] = 1;
    if (s[i] == '1' || s[i] == '?')
    {
        int nk5 = (k5 << 1) + 1;
        nk5 &= (1 << 5) - 1;
        int nk6 = (k6 << 1) + 1;
        nk6 &= (1 << 6) - 1;
        if (solve(i + 1, nk5, nk6))
        {
            return 1;
        }
    }
    if (s[i] == '0' || s[i] == '?')
    {
        int nk5 = (k5 << 1);
        nk5 &= (1 << 5) - 1;
        int nk6 = (k6 << 1);
        nk6 &= (1 << 6) - 1;
        if (solve(i + 1, nk5, nk6))
        {
            return 1;
        }
    }
    return 0;
}
int main()
{
    FASTIO
    PRECISION
    int qr;
    cin >> qr;
    for (int i = 0; i < (1 << 5); i++)
    {
        string s;
        for (int j = 0; j < 5; j++)
        {
            s += (i & (1 << j)) ? '1' : '0';
        }
        string t = s;
        reverse(iter(t));
        bad5[i] = (s == t);
    }

    for (int i = 0; i < (1 << 6); i++)
    {
        string s;
        for (int j = 0; j < 6; j++)
        {
            s += (i & (1 << j)) ? '1' : '0';
        }
        string t = s;
        reverse(iter(t));
        bad6[i] = (s == t);
    }
    for (int curr = 1; curr <= qr; curr++)
    {
        memset(dp, 0, sizeof dp);
        int n;
        cin >> n;
        cin >> s;
        bool ans = solve(0, 0, 0);
        res;
        if (!ans)
        {
            cout << "IM";
        }
        cout << "POSSIBLE" << nl;
    }
}