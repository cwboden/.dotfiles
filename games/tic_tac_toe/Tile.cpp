//
// Created by cwboden on 12/5/17.
//

#include <SDL2/SDL_events.h>
#include "Tile.h"

Tile::Tile()
{
    mPosition.x = 0;
    mPosition.y = 0;

    mCurrentSprite = BUTTON_SPRITE_MOUSE_OUT;
}

void Tile::setPosition( int x, int y )
{
    mPosition.x = x;
    mPosition.y = y;
}

void Tile::handleEvent( SDL_Event* e )
{
    //If mouse event happened
    if( e->type == SDL_MOUSEMOTION || e->type == SDL_MOUSEBUTTONDOWN || e->type == SDL_MOUSEBUTTONUP )
    {
        //Get mouse position
        int x, y;
        SDL_GetMouseState( &x, &y );

        //Check if mouse is in button
        bool inside = true;

        //Mouse is left of the button
        if( x < mPosition.x )
        {
            inside = false;
        }
            //Mouse is right of the button
        else if( x > mPosition.x + TILE_WIDTH )
        {
            inside = false;
        }
            //Mouse above the button
        else if( y < mPosition.y )
        {
            inside = false;
        }
            //Mouse below the button
        else if( y > mPosition.y + TILE_HEIGHT )
        {
            inside = false;
        }

        //Mouse is outside button
        if( !inside )
        {
            mCurrentSprite = BUTTON_SPRITE_MOUSE_OUT;
        }
            //Mouse is inside button
        else
        {
            //Set mouse over sprite
            switch( e->type )
            {
                case SDL_MOUSEMOTION:
                    mCurrentSprite = BUTTON_SPRITE_MOUSE_OVER_MOTION;
                    break;

                case SDL_MOUSEBUTTONDOWN:
                    mCurrentSprite = BUTTON_SPRITE_MOUSE_DOWN;
                    break;

                case SDL_MOUSEBUTTONUP:
                    mCurrentSprite = BUTTON_SPRITE_MOUSE_UP;
                    break;
            }
        }
    }
}

void Tile::render()
{
    //Show current button sprite
    gTileSpriteSheetTexture.render( mPosition.x, mPosition.y, &gSpriteClips[ mCurrentSprite ] );
}
