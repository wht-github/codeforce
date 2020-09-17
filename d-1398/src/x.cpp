// 本题为考试单行多行输入输出规范示例，无需提交，不计分。
#include <iostream>
#include <vector>
using namespace std;
static long long M = 10e9 + 7;
int main()
{
    long long n, l, r;
    cin >> n >> l >> r;
    long long arr[2][3];
    long long *cur = arr[0];
    long long *next = arr[1];
    //next[0] = 0;
    //next[1] = 0;
    //next[2] = 0;
    for (long long i = 0; i < 2; i++)
    {
        for (long long j = 0; j < 3; j++)
        {
            arr[i][j] = 0;
        }
    }
    for (long long j = 0; j < r - l + 1; j++)
    {
        long long tmp = l + j;
        cur[tmp % 3] += 1;
        cur[tmp % 3] %= M;
    }
    for (long long i = 1; i < n; i++)
    {
        for (long long j = 0; j < r - l + 1; j++)
        {
            long long tmp = l + j;
            for (long long k = 0; k < 3; k++)
            {
                if (cur[k] == 0)
                    continue;
                next[(tmp % 3 + k) % 3] += cur[k];
                next[(tmp % 3 + k) % 3] %= M;
            }
        }
        long long *t = cur;
        cur = next;
        next = t;
        for (long long j = 0; j < 3; j++){
            next[j] = 0;
        }
    }
    cout << cur[0] << endl;
}