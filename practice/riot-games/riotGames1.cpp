// Carson Boden / October 2016
// Finds optimal plays

using namespace std;
int main() {

    struct Entry
    {
        int endTime;
        int startTime;
        int t1Gold;
        int t2Gold;
        bool operator<(const Entry &other) const
        {
            return endTime < other.endTime;
        }
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
        // Time
        cin >> entry.endTime;
        // Team 1 Gold
        cin >> entry.t1Gold;
        // Team 2 Gold
        cin >> entry.t2Gold;

        timeline.push_back(entry);
    }

    Entry start = Entry{0, 0, 0};
    timeline.push_back(start);

    sort(timeline.begin(), timeline.end());

    double goldDt = 0;
    vector<Entry> output;

    for (size_t i = 1; i < timeline.size(); ++i)
    {
        int t1Dt = (timeline[i].t1Gold - timeline[i-1].t1Gold) /
            (timeline[i].endTime - timeline[i-1].endTime);
        int t2Dt = (timeline[i].t2Gold - timeline[i-1].t2Gold) /
            (timeline[i].endTime - timeline[i-1].endTime);

        if (goldDt <= abs(t1Dt - t2Dt))
        {
            if (goldDt != abs(t1Dt - t2Dt))
            {
                goldDt = abs(t1Dt - t2Dt);
                output.clear();
            }
            timeline[i].startTime = timeline[i-1].endTime;
            if (!output.empty() &&
                output.back().endTime == timeline[i].startTime)
            {
                output.back().endTime = timeline[i].endTime;
            }
            else
            {
                output.push_back(timeline[i]);
            }
        }
    }

    for (size_t i = 0; i < output.size(); ++i)
    {
        cout << output[i].startTime << ' ' << output[i].endTime << ' ' <<
            fixed << setprecision(2) << goldDt << '\n';
    }

    return 0;
}
