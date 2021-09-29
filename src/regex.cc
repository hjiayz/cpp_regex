#include "cpp_regexp/include/regex.hpp"
#include <iostream>
#include <locale>

Regex::Regex(std::string const &s, std::regex_constants::syntax_option_type pflag, std::regex_constants::match_flag_type pmatch_flag, std::string const &locale)
{
  pattern.imbue(std::locale(locale));
  pattern.assign(s, pflag);
  match_flag = pmatch_flag;
}

bool Regex::test(std::string const &s) const
{
  return std::regex_search(s, pattern, match_flag);
}

rust::String Regex::replace(std::string const &s, std::string const &replacement) const
{
  return rust::String(std::regex_replace(s, pattern, replacement, match_flag));
}

rust::Vec<rust::String> Regex::regex_match(std::string const &s) const
{
  std::smatch match;
  std::regex_match(s, match, pattern, match_flag);
  rust::Vec<rust::String> results;
  for (size_t i = 0; i < match.size(); i++)
  {
    results.push_back(rust::String(match[i].str()));
  }
  return results;
};

void Regex::match_all(std::string const &s,rust::Vec<rust::String> &results, rust::Vec<size_t> &offsets) const{
  auto begin = std::sregex_iterator(s.begin(), s.end(),pattern,match_flag);
  auto end = std::sregex_iterator();
  size_t offset = 0;
  offsets.push_back(offset);
  for (auto i = begin; i != end; ++i)
  {
    std::smatch match = *i;
    size_t size = match.size();
    offset = offset + size;
    offsets.push_back(offset);
    for (size_t j = 0; j < size; j++)
    {
      std::string match_str = match[j].str(); 
      results.push_back(rust::String(match_str));
    }
  }
  return;
}

std::unique_ptr<Regex> new_regex(
    std::string const &s,
    bool icase,
    bool nosubs,
    bool optimize,
    bool collate,
    size_t grammar,
    bool format_no_copy,
    bool format_first_only,
    bool format_sed,
    bool match_not_bol,
    bool match_not_eol,
    bool match_not_bow,
    bool match_not_eow,
    bool match_any,
    bool match_not_null,
    bool match_continuous,
    bool match_prev_avail,
    std::string const &locale)
{
  const std::regex_constants::syntax_option_type map[6] = {
      std::regex_constants::ECMAScript,
      std::regex_constants::basic,
      std::regex_constants::extended,
      std::regex_constants::awk,
      std::regex_constants::grep,
      std::regex_constants::egrep,
  };
  auto flag = map[grammar];
  if (icase)
  {
    flag = flag | std::regex_constants::icase;
  }
  if (nosubs)
  {
    flag = flag | std::regex_constants::nosubs;
  }
  if (optimize)
  {
    flag = flag | std::regex_constants::optimize;
  }
  if (collate)
  {
    flag = flag | std::regex_constants::collate;
  }
  auto match_flag = std::regex_constants::format_default;
  if (format_sed)
  {
    match_flag = std::regex_constants::format_sed;
  }
  if (format_no_copy)
  {
    match_flag = match_flag | std::regex_constants::format_no_copy;
  }
  if (format_first_only)
  {
    match_flag = match_flag | std::regex_constants::format_first_only;
  }
  if (match_not_bol)
  {
    match_flag = match_flag | std::regex_constants::match_not_bol;
  }
  if (match_not_eol)
  {
    match_flag = match_flag | std::regex_constants::match_not_eol;
  }
  if (match_not_bow)
  {
    match_flag = match_flag | std::regex_constants::match_not_bow;
  }
  if (match_not_eow)
  {
    match_flag = match_flag | std::regex_constants::match_not_eow;
  }
  if (match_any)
  {
    match_flag = match_flag | std::regex_constants::match_any;
  }
  if (match_not_null)
  {
    match_flag = match_flag | std::regex_constants::match_not_null;
  }
  if (match_continuous)
  {
    match_flag = match_flag | std::regex_constants::match_continuous;
  }
  if (match_prev_avail)
  {
    match_flag = match_flag | std::regex_constants::match_prev_avail;
  }
  Regex regex(s, flag, match_flag, locale);
  return std::make_unique<Regex>(regex);
}