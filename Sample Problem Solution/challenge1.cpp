#include <iostream>
using namespace std;
int main()
{
    long int t, m, n, candies, leftCandies;
    cin >> t;
    for (int i = 0; i <= t; ++i)
    {
        cin >> m;
        cin >> n;
        candies = 0;
        for (int j = 1; j <= n; j++)
        {
            cin >> candies + candies;
        }
        leftCandies = candies % m;
        cout << "Case #" << i << ": " << leftCandies;
    }
    return 0;
}