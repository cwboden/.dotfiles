// Carson Boden / October 2016
// Finds optimal plays

using namespace std;
int main() {
    /* Enter your code here. Read input from STDIN. Print output to STDOUT */

    struct Entry
    {
        int ts;
    };

    size_t numEntries;
    vector<Entry> timeline;

    if (!cin.good())
    {
        return 1;
    }

    cin >> numEntries;

    for (size_t i = 0; i < numEntries; ++i)
    {
        Entry entry;
        int trash;
        // Time
        cin >> entry.ts;
        cin >> trash;
        cin >> trash;

        timeline.push_back(entry);
    }

    int maxKills;
    vector<Entry> output;

    for (size_t i = 0; i < timeline.size(); ++i)
    {
        int timeLimit = timeline[i].ts + 30;
        int numKills = 0;
        for(size_t j = 0; j < timeline.size(); ++j)
        {
            if (timeline[j].ts < timeLimit && timeline[j].ts >= timeline[i].ts)
            {
                numKills++;
            }
        }

        if (maxKills <= numKills)
        {
            if (maxKills != numKills)
            {
                maxKills = numKills;
                output.clear();
            }

            output.push_back(timeline[i]);
        }
    }

    for (size_t i = 0; i < output.size(); ++i)
    {
        cout << output[i].ts << ' ' << maxKills << '\n';
    }

    return 0;
}
