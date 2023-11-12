#include <iostream>
#include <string>
#include <fstream>
#include <cassert>
using namespace std;

int fuel_required(int mass)
{
    return (mass / 3) - 2;
}

int main()
{

    assert(fuel_required(12) == 2);
    assert(fuel_required(14) == 2);
    assert(fuel_required(1969) == 654);
    assert(fuel_required(100756) == 33583);

    ifstream f = ifstream("./input.txt");
    int mass;
    int total = 0;

    while (f >> mass)
    {
        total += fuel_required(mass);
    }

    cout << total << endl;

    return 0;
}