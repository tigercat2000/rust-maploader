// This file provides manually written utility types and such for the BAPI DMM Reader
// and imports the bindings 

// Compatibility for non-ss13
#include "compat.dm"

// Import bindings to the rust library
#include "bapi_bindings.dm"

#define MAP_DMM "dmm"
/**
 * TGM SPEC:
 * TGM is a derevation of DMM, with restrictions placed on it
 * to make it easier to parse and to reduce merge conflicts/ease their resolution
 *
 * Requirements:
 * Each "statement" in a key's details ends with a new line, and wrapped in (...)
 * All paths end with either a comma or occasionally a {, then a new line
 * Excepting the area, who is listed last and ends with a ) to mark the end of the key
 *
 * {} denotes a list of variable edits applied to the path that came before the first {
 * the final } is followed by a comma, and then a new line
 * Variable edits have the form \tname = value;\n
 * Except the last edit, which has no final ;, and just ends in a newline
 * No extra padding is permitted
 * Many values are supported. See parse_constant()
 * Strings must be wrapped in "...", files in '...', and lists in list(...)
 * Files are kinda susy, and may not actually work. buyer beware
 * Lists support assoc values as expected
 * These constants can be further embedded into lists
 *
 * There can be no padding in front of, or behind a path
 *
 * Therefore:
 * "key" = (
 * /path,
 * /other/path{
 *     var = list("name" = 'filepath');
 *     other_var = /path
 *     },
 * /turf,
 * /area)
 *
 */
#define MAP_TGM "tgm"
#define MAP_UNKNOWN "unknown"

/// Returned from parse_map to give some metadata about the map
/datum/bapi_parsed_map
	var/_internal_index = -1

	var/original_path = ""
	var/map_format = MAP_UNKNOWN
	var/key_len = 0
	var/line_len = 0
	var/expanded_y = 0
	var/expanded_x = 0

	var/list/bounds = list()
	var/list/parsed_bounds = list()

	var/loading = FALSE

/**
 * Helper and recommened way to load a map file
 * - dmm_file: The path to the map file
 * - x_offset: The x offset to load the map at
 * - y_offset: The y offset to load the map at
 * - z_offset: The z offset to load the map at
 * - crop_map: If true, the map will be cropped to the world bounds
 * - measure_only: If true, the map will not be loaded, but the bounds will be calculated
 * - no_changeturf: If true, the map will not call /turf/AfterChange
 * - x_lower: The minimum x coordinate to load
 * - x_upper: The maximum x coordinate to load
 * - y_lower: The minimum y coordinate to load
 * - y_upper: The maximum y coordinate to load
 * - z_lower: The minimum z coordinate to load
 * - z_upper: The maximum z coordinate to load
 * - place_on_top: Whether to use /turf/proc/PlaceOnTop rather than /turf/proc/ChangeTurf
 * - new_z: If true, a new z level will be created for the map
 */
/proc/load_map(
	dmm_file,
	x_offset = 0,
	y_offset = 0,
	z_offset = 0,
	crop_map = FALSE,
	measure_only = FALSE,
	no_changeturf = FALSE,
	x_lower = -INFINITY,
	x_upper = INFINITY,
	y_lower = -INFINITY,
	y_upper = INFINITY,
	z_lower = -INFINITY,
	z_upper = INFINITY,
	place_on_top = FALSE,
	new_z = FALSE,
)
	if(!(dmm_file in cached_maps))
		cached_maps[dmm_file] = new /datum/bapi_parsed_map(dmm_file)

	var/datum/bapi_parsed_map/parsed_map = cached_maps[dmm_file]
	parsed_map = parsed_map.copy()
	if(!measure_only && !isnull(parsed_map.bounds))
		parsed_map.load(x_offset, y_offset, z_offset, crop_map, no_changeturf, x_lower, x_upper, y_lower, y_upper, z_lower, z_upper, place_on_top, new_z)
	return parsed_map

/datum/bapi_parsed_map/New(tfile)
	if(isnull(tfile))
		return // create a new datum without loading a map
	var/ret = _bapidmm_parse_map_blocking(tfile, src)
	if(!ret)
		CRASH("Failed to load map [tfile], check rust_log.txt")

/datum/bapi_parsed_map/proc/copy()
	// Avoids duped work just in case
	build_cache()
	var/datum/bapi_parsed_map/newfriend = new()
	// use the same under the hood data
	newfriend._internal_index = _internal_index
	newfriend.original_path = original_path
	newfriend.map_format = map_format
	newfriend.key_len = key_len
	newfriend.line_len = line_len
	// newfriend.grid_models = grid_models.Copy()
	// newfriend.gridSets = gridSets.Copy()
	// newfriend.modelCache = modelCache.Copy()
	newfriend.parsed_bounds = parsed_bounds.Copy()
	// Copy parsed bounds to reset to initial values
	newfriend.bounds = parsed_bounds.Copy()
	// newfriend.turf_blacklist = turf_blacklist?.Copy()
	return newfriend

/datum/bapi_parsed_map/proc/build_cache()
	return

/datum/bapi_parsed_map/proc/load(
	x_offset = 0,
	y_offset = 0,
	z_offset = 0,
	crop_map = FALSE,
	measure_only = FALSE,
	no_changeturf = FALSE,
	x_lower = -INFINITY,
	x_upper = INFINITY,
	y_lower = -INFINITY,
	y_upper = INFINITY,
	z_lower = -INFINITY,
	z_upper = INFINITY,
	place_on_top = FALSE,
	new_z = FALSE,
)
	return