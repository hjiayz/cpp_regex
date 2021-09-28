#include "cpp_regexp/include/regex.hpp"


Regex::Regex(std::string const &s,std::regex_constants::syntax_option_type pflag,std::regex_constants::match_flag_type pmatch_flag) {
  pattern = std::regex(s,pflag);
  match_flag = pmatch_flag;
}

bool Regex::test(std::string const &s) const{
  return std::regex_search(s,pattern,match_flag);
}

std::unique_ptr<std::string> Regex::replace(std::string const &s,std::string const &replacement) const{
  return std::make_unique<std::string>(std::regex_replace(s,pattern,replacement,match_flag));
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
  bool match_prev_avail) {
  const std::regex_constants::syntax_option_type map[6] = {
      std::regex::ECMAScript,
      std::regex::basic,
      std::regex::extended,
      std::regex::awk,
      std::regex::grep,
      std::regex::egrep,
  };
  auto flag = map[grammar];
  if (icase) {
      flag = flag | std::regex::icase;
  }
  if (nosubs) {
      flag = flag | std::regex::nosubs;
  }
  if (optimize) {
      flag = flag | std::regex::optimize;
  }
  if (collate) {
      flag = flag | std::regex::collate;
  }
  auto match_flag = std::regex_constants::format_default;
  if (format_sed) {
    match_flag = std::regex_constants::format_sed;
  }
  if (format_no_copy) {
    match_flag = match_flag | std::regex_constants::format_no_copy;
  }
  if (format_first_only) {
    match_flag = match_flag | std::regex_constants::format_first_only;
  }
  if (match_not_bol) {
    match_flag = match_flag | std::regex_constants::match_not_bol;
  }
  if (match_not_eol) {
    match_flag = match_flag | std::regex_constants::match_not_eol;
  }
  if (match_not_bow) {
    match_flag = match_flag | std::regex_constants::match_not_bow;
  }
  if (match_not_eow) {
    match_flag = match_flag | std::regex_constants::match_not_eow;
  }
  if (match_any) {
    match_flag = match_flag | std::regex_constants::match_any;
  }
  if (match_not_null) {
    match_flag = match_flag | std::regex_constants::match_not_null;
  }
  if (match_continuous) {
    match_flag = match_flag | std::regex_constants::match_continuous;
  }
  if (match_prev_avail) {
    match_flag = match_flag | std::regex_constants::match_prev_avail;
  }
  Regex regex(s,flag,match_flag);
  return std::make_unique<Regex>(regex);
}