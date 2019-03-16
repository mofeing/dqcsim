%module dqcsim
%{
#include "dqcsim.h"
%}

%include c/gen/dqcsim.h

// Mark all functions that return a newly allocated object with newobject, so
// swig deallocates them after making its own copy.
%newobject dqcs_arb_get_str;
%newobject dqcs_arb_json_get_str;
%newobject dqcs_arb_pop_str;
%newobject dqcs_handle_dump;

%inline %{
typedef long unsigned int size_t;
typedef long signed int ssize_t;
%}