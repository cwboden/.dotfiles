/**
 * PROMPT:
 * In the board game 7 Wonders, there are four basic resources, which we'll
 * abbreviate with letters: W for wood, B for brick, S for stone, and O for ore.
 *
 * Resource cards let you produce one of your choice of resources. We'll use W/B
 * to represent a resource card that can give you either 1 wood or 1 brick, but
 * not both.
 *
 * Given the resource cards W/B/S/O, W, S/B, and S, it is possible for you to
 * produce 2 wood and 2 stone: use the first two cards to get wood, and the last
 * two to get stone. However, with that same set of cards, it is impossible for
 * you to produce 2 wood and 2 brick.
 *
 * You'll be given a comma-separated sequence of cards inside of square brackets,
 * with the features separated by a slash. Your target will be given as "Can you
 * make ____?" with the list of resources to target, one per card.
 *
 * Output whether it is possible to generate the desired resources, and if so, how.
 *
 * @author Carson Boden
 */

#include <algorithm>
#include <iostream>
#include <set>
#include <string>
#include <vector>

class Card
{
 public:
  int id;
  std::set<char> resources;
  std::vector<char> resources_ordered;
  char current_use;

  Card(std::string resource_str) {
    id = id_counter++;
    current_use = '-';

    resource_str.erase(
        std::remove(resource_str.begin(), resource_str.end(), '/'),
        resource_str.end());

    for (auto resource : resource_str) {
      resources_ordered.push_back(resource);
      resources.insert(resource);
    }
  }

  void print_details()
  {
    std::cout << "Card " << id << " [";

    for (int i = 0; i < resources_ordered.size(); ++i) {
      std::cout << resources_ordered[i];

      if (i < resources_ordered.size() - 1) {
        std::cout << '/';
      }
    }

    std::cout << "] -> " << current_use << std::endl;
  }

 private:
  static int id_counter;
};

std::string get_query(std::string input);
std::vector<Card> get_cards(std::string input);
bool can_find_resources(std::vector<Card>& cards, std::string query, int index = 0);

int main(int argc, char* argv[])
{
  std::string input;
  std::getline(std::cin, input);

  std::string query = get_query(input);
  std::vector<Card> cards = get_cards(input);

  if (can_find_resources(cards, query)) {
    std::cout << query << " possible with given cards." << std::endl;
    for (auto& card : cards) {
      card.print_details();
    }
  } else {
    std::cout << query << " not possible with given cards." << std::endl;
  }

  return 0;
}

int Card::id_counter = 0;

std::string get_query(std::string input)
{
  int query_index = input.find("Can you make ") + 13;
  std::string query_str =
      input.substr(query_index, input.length() - query_index - 1);

  return query_str;
}

std::vector<Card> get_cards(std::string input)
{
  int cards_start_index = input.find("[") + 1;
  int cards_end_index = input.find("]");
  std::string cards_str =
      input.substr(cards_start_index, cards_end_index - cards_start_index);

  std::vector<Card> cards;
  int comma_index = -1;
  do {
    comma_index = cards_str.find(",");
    std::string card_str = cards_str.substr(0, comma_index);

    Card card = Card(card_str);
    cards.push_back(card);

    cards_str = cards_str.substr(comma_index + 2);
  } while (comma_index != -1);

  return cards;
}

bool can_find_resources(std::vector<Card>& cards, std::string query, int index)
{
  if (query.empty()) {
    return true;
  }

  auto& card = cards.at(index);
  for (auto resource : card.resources) {
    int pos = query.find(resource);
    if (pos != -1) {
      card.current_use = resource;

      std::string new_query = query.substr(0, pos) + query.substr(pos + 1);
      if (can_find_resources(cards, new_query, index + 1)) {
        return true;
      }
    }
  }

  return false;
}
