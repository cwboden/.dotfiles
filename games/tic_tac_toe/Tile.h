//
// Created by cwboden on 12/5/17.
//

#ifndef TIC_TAC_TOE_TILE_H
#define TIC_TAC_TOE_TILE_H


#include <SDL2/SDL_rect.h>

#include "Texture.h"

//Tile constants
const int TILE_WIDTH = 160;
const int TILE_HEIGHT = 160;

extern SDL_Rect gSpriteClips[];
extern Texture gTileSpriteSheetTexture;

enum TileSprite
{
    BUTTON_SPRITE_MOUSE_OUT = 0,
    BUTTON_SPRITE_MOUSE_OVER_MOTION = 1,
    BUTTON_SPRITE_MOUSE_DOWN = 2,
    BUTTON_SPRITE_MOUSE_UP = 3,
    BUTTON_SPRITE_TOTAL = 4
};

//The mouse button
class Tile
{
public:
    //Initializes internal variables
    Tile();

    //Sets top left position
    void setPosition( int x, int y );

    //Handles mouse event
    void handleEvent( SDL_Event* e );

    //Shows button sprite
    void render();

private:
    //Top left position
    SDL_Point mPosition;

    //Currently used global sprite
    TileSprite mCurrentSprite;
};

#endif //TIC_TAC_TOE_TILE_H
