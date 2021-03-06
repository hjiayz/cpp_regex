#pragma once
#include "rust/cxx.h"
#include <string>
#include <regex>

struct MatchItem;
struct MatchGroup;

class Regex
{
public:
    std::regex pattern;
    std::regex_constants::match_flag_type match_flag;
    Regex(std::string const &s, std::regex_constants::syntax_option_type pflag, std::regex_constants::match_flag_type pmatch_flag, std::string const &locale);
    bool test(std::string const &s) const;
    rust::String replace(std::string const &s, std::string const &replacement) const;
    MatchGroup regex_match(rust::Str s) const;
    rust::Vec<MatchGroup> match_all(rust::Str s) const;
};

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
    std::string const &locale);
