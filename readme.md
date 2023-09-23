# Pixel Processing Unit SIMulate

## old 8 bit console style

rgab r,g,b,a 각 8bit (a 는 미사용) 

palette : 256 개 의 sub palette 로 구성 

    file로 읽고 쓸때는 256x256xRGBA bmp 를 사용

sub palette : 256 개의 rgba  

tile : 8 x 8 크기, 각 구성 요소는 sub palette index (8bit 크기) 를 가진다.  

tile_vec : tile의 목록,  256 page 의 256 tile 

    file로 읽고 쓸때는 2048x2048 256gray bmp사용 
    각 페이지는 16x16 구역으로 나뉘어 진다. 

tilemap : sprite나 background image 를 구성하기 위한 정보를 담고있다. 

    pos : 좌표 : (signed 16bit)x2  
    wh : 크기 : (unsigned 8bit)x2 가로 세로 타일 수  
    scale : 확대/flip x,y : scale < 0 으로 하면 flip 된다.
    upper_palette_index : 사용할 palette page 8bit, 여러 tilemap 이 palette를 공유가능하다.
    upper_tilevec_index : 사용할 tile_vec page 8bit, 여러 tilemap 이 tile_vec page를 공유가능하다.
    tilemap_buffer_index : tilemap buffer 에서 사용할 타일 목록 시작 지점 
        크기(wh) 만큼의 tile을 사용한다. 
    enable : on/off

tilemap_vec : tilemap의 목록 : 이것을 사용해서 화면을 그린다. 

tilemap_buffer : tilemap에서 사용할 tile 번호들의 목록 

    tilemap 의 tilemap_buffer_index 에서 시작애서  크기(wh)만큼을 사용한다. 
    각 tilemap에서 사용하는 tile 은 서로 겹칠 수 있다. 
    ( 같은 tilemap_buffer 공간을 공유 가능, 따라서 tile도 공유가능 ) 
    tilemap의 upper_tilevec_index  와 여기의 tile index 를 사용해서 tile_vec의 위치를 찾는다. 

tilesdef.bmp 는 
https://opengameart.org/content/dungeon-crawl-32x32-tiles
에서 가져온 것 이다. 