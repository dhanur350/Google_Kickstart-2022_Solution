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

// position, > a, < b, first, prod % x, sum
int a[15], b[15];
int x;
ll dp[15][3][3][3][120][120];
int vis[15][3][3][3][120][120];
int vx = 1;
ll solve(int p, int aok, int bok, int f, int prod, int sum)
{
    if (p == -1)
    {
        return prod == 0 && sum == x;
    }
    ll &r = dp[p][aok][bok][f][prod][sum];
    if (vis[p][aok][bok][f][prod][sum] == vx)
    {
        return r;
    }
    vis[p][aok][bok][f][prod][sum] = vx;
    r = 0;
    // no leading zeroes
    for (int d = 0; d <= 9; d++)
    {
        if (!aok && d < a[p])
            continue;
        if (!bok && d > b[p])
            continue;
        int naok = aok || d > a[p];
        int nbok = bok || d < b[p];
        int nf = f && d == 0;
        int nprod = (prod * d) % x;
        if (nf)
        {
            nprod = 1;
        }
        int nsum = (sum + d);
        r += solve(p - 1, naok, nbok, nf, nprod, nsum);
    }
    return r;
}
int brute_force(int a, int b)
{
    int ct = 0;
    for (int i = a; i <= b; i++)
    {
        ll sum = 0, prod = 1;
        int k = i;
        while (k)
        {
            int d = k % 10;
            sum += d;
            prod *= d;
            k /= 10;
        }
        ct += !(prod % sum);
    }
    return ct;
}
int main()
{
    FASTIO
    PRECISION
    int qr;
    cin >> qr;
    for (int curr = 1; curr <= qr; curr++)
    {
        string av, bv;
        cin >> av >> bv;
        memset(a, 0, sizeof a);
        memset(b, 0, sizeof b);
        for (int i = 0; i < (int)av.size(); i++)
        {
            a[i] = av[av.size() - i - 1] - '0';
        }
        for (int i = 0; i < (int)bv.size(); i++)
        {
            b[i] = bv[bv.size() - i - 1] - '0';
        }
        ll ans = 0;
        for (x = 1; x <= 108; x++)
        {
            vx++;
            ans += solve(12, 0, 0, 1, 1, 0);
        }
        cout << "Case #" << curr << ": " << ans << nl;
        // dbn(brute_force(stoi(av), stoi(bv)));
    }
}