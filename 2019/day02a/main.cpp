#include <vector>
#include <fstream>
#include <iostream>
#include <cassert>
using namespace std;

int execute_instruction(vector<int> instructions)
{
    int instruction_pointer = 0;
    while (true)
    {
        bool terminated = false;

        int opcode = instructions[instruction_pointer];
        if (opcode == 99)
        {
            break;
        }
        else if (opcode == 1)
        {

            int operand_a = instructions[instruction_pointer + 1];
            int operand_b = instructions[instruction_pointer + 2];
            int output = instructions[instruction_pointer + 3];

            instructions[output] = instructions[operand_a] + instructions[operand_b];

            instruction_pointer += 4;
        }
        else if (opcode == 2)
        {
            int operand_a = instructions[instruction_pointer + 1];
            int operand_b = instructions[instruction_pointer + 2];
            int output = instructions[instruction_pointer + 3];

            instructions[output] = instructions[operand_a] * instructions[operand_b];

            instruction_pointer += 4;
        }
        else
        {
            throw invalid_argument("Invalid opcode");
        };
    }

    return instructions[0];
}

int main()
{
    vector<int> int_code;

    fstream f = fstream("./input.txt");

    string instruction;
    while (getline(f, instruction, ','))
    {
        int_code.push_back(stoi(instruction));
    }

    assert(execute_instruction({1, 0, 0, 0, 99}) == 2);
    assert(execute_instruction({1, 1, 1, 4, 99, 5, 6, 0, 99}) == 30);
    assert(execute_instruction({1, 1, 1, 4, 99, 5, 6, 0, 99}) == 30);

    // apply the patches as described in the problem
    int_code[1] = 12;
    int_code[2] = 2;

    cout
        << execute_instruction(int_code) << endl;

    return 0;
}