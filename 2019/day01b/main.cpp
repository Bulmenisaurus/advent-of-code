#include <iostream>
#include <string>
#include <fstream>
#include <cassert>
using namespace std;

int fuel_required(int mass)
{
    int partial_fuel = (mass / 3) - 2;

    if (partial_fuel <= 0)
    {
        return 0;
    }

    return partial_fuel + fuel_required(partial_fuel);
}

int main()
{

    assert(fuel_required(14) == 2);
    assert(fuel_required(1969) == 966);
    assert(fuel_required(100756) == 50346);

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