#include <iostream>
#include <fstream>
#include <string>
#include <vector>

using namespace std;

struct LineSegment
{
    bool horizontal;
    int startX;
    int startY;

    int endX;
    int endY;
};

vector<LineSegment> get_segment(string line)
{
    // split by commas
    vector<string> sections;
    string token;
    size_t pos = 0;
    while ((pos = line.find(",")) != std::string::npos)
    {
        token = line.substr(0, pos);
        sections.push_back(token);
        line.erase(0, pos + 1);
    }
    sections.push_back(token);

    vector<LineSegment> segments;
    int x = 0;
    int y = 0;

    for (int i = 0; i < sections.size(); i++)
    {
        char direction = sections[i][0];
        int distance = atoi(sections[i].substr(1).c_str());

        int startX = x, startY = y;

        switch (direction)
        {
        case 'U':
            y += distance;
            break;

        case 'R':
            x += distance;
            break;

        case 'D':
            y -= distance;
            break;

        case 'L':
            x -= distance;
            break;
        }

        LineSegment segment;
        segment.startX = startX;
        segment.startY = startY;
        segment.endX = x;
        segment.endY = y;
    }

    return segments;
}

vector<pair<int, int>> get_intersections(vector<LineSegment> line1, vector<LineSegment> line2)
{
    vector<pair<int, int>> n;
    return n;
}

int closest_point(vector<pair<int, int>> points)
{
    int n = 0;
    return n;
}

int get_closest_intersection(string line1, string line2)
{
    vector<LineSegment> line1Segments = get_segment(line1);
    vector<LineSegment> line2Segments = get_segment(line2);

    vector<pair<int, int>> intersections = get_intersections(line1Segments, line2Segments);

    int closestIntersection = closest_point(intersections);

    return closestIntersection;
}

int main()
{
    fstream f = fstream("./input.txt");
    string line1, line2;

    f >> line1 >> line2;

    get_closest_intersection(line1, line2);

    return 0;
}