// Copyright (C) 2007-2012 Nikolaus Gebhardt
// This file is part of the "Irrlicht Engine".
// For conditions of distribution and use, see copyright notice in irrlicht.h

// include this file right before the data structures to be 1-aligned
// and add to each structure the PACK_STRUCT define just like this:
// struct mystruct
// {
//	...
// } PACK_STRUCT;
// Always include the irrunpack.h file right after the last type declared
// like this, and do not put any other types with different alignment
// in between!

// byte-align structures
#if defined(__GNUC__)
#define PACK_STRUCT __attribute__((packed))
#else
#error compiler not supported
#endif
