/**
 * @file: element.h
 *
 * Introduces the element class and can scan the resources folder to return
 * different data structures composed of elements.
 *
 * @author Carson Boden
 */

#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <vector>

class Element
{
 public:
  Element() : Element("", "", 0) { }
  Element(std::string name_, std::string symbol_, float atomic_weight_) :
      name(name_), symbol(symbol_), atomic_weight(atomic_weight_) { }

  void Print()
  {
    std::cout << name << " (" << symbol << "): " << atomic_weight << std::endl;
  }

  std::string name;
  std::string symbol;
  float atomic_weight;
};

std::vector<Element> get_elements()
{
  std::vector<Element> elements;
  std::ifstream file_stream("../resources/ptdata2.csv");
  std::string trash, symbol, element, weight;

  // Order of CSV File:
  // Atomic Number, Symbol, Element, Weight
  getline(file_stream, trash);
  while (file_stream.good()) {
    getline(file_stream, trash, ',');
    getline(file_stream, symbol, ',');
    getline(file_stream, element, ',');
    getline(file_stream, weight);

    elements.push_back(Element(element, symbol, stof(weight)));
  }

  return elements;
}

std::map<std::string, Element> get_symbol_elements_map()
{
  std::vector<Element> elements = get_elements();
  std::map<std::string, Element> elements_map;

  for (Element& element : elements) {
    elements_map[element.symbol] = element;
  }

  return elements_map;
}
