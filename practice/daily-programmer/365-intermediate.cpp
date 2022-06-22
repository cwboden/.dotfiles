/**
 * PROMPT:
 * You're a regional manager for an office beverage sales company, and right now
 * you're in charge of paying your sales team they're monthly commissions.
 *
 * Sales people get paid using the following formula for the total commission:
 * commission is 6.2% of profit, with no commission for any product to total
 * less than zero.
 *
 * @author Carson Boden
 */

#include <iostream>
#include <vector>

std::vector<double> calculate_commissions(
    std::vector<std::vector<int>> revenue,
    std::vector<std::vector<int>> expenses);

void print_commissions(std::vector<double> commissions);

int main(int argc, char* argv[])
{
  // Generate test data
  std::vector<std::vector<int>> standard_revenue = {
    { 120, 145 },
    { 243, 265 }
  };
  std::vector<std::vector<int>> standard_expenses = {
    { 130, 59 },
    { 143, 198 }
  };

  std::vector<std::vector<int>> challenge_revenue = {
    { 190, 140, 1926, 14, 143 },
    { 325, 19, 293, 1491, 162 },
    { 682, 14, 852, 56, 659 },
    { 829, 140, 609, 120, 87 }
  };
  std::vector<std::vector<int>> challenge_expenses = {
    { 120, 65, 890, 54, 430 },
    { 300, 10, 23, 802, 235 },
    { 50, 299, 1290, 12, 145 },
    { 67, 254, 89, 129, 76 }
  };

  std::vector<double> standard_commissions =
      calculate_commissions(standard_revenue, standard_expenses);
  std::vector<double> challenge_commissions =
      calculate_commissions(challenge_revenue, challenge_expenses);

  print_commissions(standard_commissions);
  print_commissions(challenge_commissions);

  return 0;
}

std::vector<double> calculate_commissions(
    std::vector<std::vector<int>> revenue,
    std::vector<std::vector<int>> expenses)
{
  static const double kCommissionRate = 0.062;

  std::vector<double> commissions;

  for (int c = 0; c < revenue.front().size(); ++c) {
    double commission = 0;
    for (int r = 0; r < revenue.size(); ++r) {
      // Calculate profit
      int profit = revenue[r][c] - expenses[r][c];
      if (profit > 0) {
        commission += profit * kCommissionRate;
      }
    }
    commissions.push_back(commission);
  }

  return commissions;
}

void print_commissions(std::vector<double> commissions)
{
  std::cout << "Commission:\t";
  for (auto& commission : commissions) {
    std::cout << commission << '\t';
  }
  std::cout << std::endl;
}
