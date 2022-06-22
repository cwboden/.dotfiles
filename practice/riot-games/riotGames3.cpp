// Carson Boden / October 2016
// Finds optimal plays

using namespace std;
int main() {
    /* Enter your code here. Read input from STDIN. Print output to STDOUT */

    struct Entry
    {
        int ts;
        int killerId;
        int numKills = 0;

        bool operator<(const Entry &other) const
        {
            return ts < other.ts;
        }
    };

    size_t numEntries;
    vector<Entry> timeline;

    cin >> numEntries;

    for (size_t i = 0; i < numEntries; ++i)
    {
        Entry entry;
        int trash;

        cin >> entry.ts;
        cin >> entry.killerId;
        cin >> trash;

        timeline.push_back(entry);
    }

    sort(timeline.begin(), timeline.end());

    int maxKills;
    vector<Entry> output;

    for (size_t i = 0; i < timeline.size(); ++i)
    {
        int timeLimit = timeline[i].ts + 15;
        for(size_t j = 0; j < timeline.size(); ++j)
        {
            if (timeline[j].ts < timeLimit && timeline[j].ts >= timeline[i].ts
                && timeline[j].killerId == timeline[i].killerId)
            {
                timeline[i].numKills++;
            }
        }

        if (maxKills <= timeline[i].numKills)
        {
            if (maxKills != timeline[i].numKills)
            {
                maxKills = timeline[i].numKills;
                output.clear();
            }

            if (find(output.begin(), output.end(), timeline[i]) == output.end())
            {
                output.push_back(timeline[i]);
            }
        }
    }

    for (size_t i = 0; i < output.size(); ++i)
    {
        cout << maxKills << ' ' << output[i].killerId << ' ' << output[i].ts << '\n';
    }

    return 0;
}
