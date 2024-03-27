REMOVE TABLE review;
REMOVE TABLE artist;

CREATE review;
CREATE artist;

UPDATE review:00a0ic854u4j7z02s00v 
CONTENT { artist: artist:00b2pg847d7b8r08t08t, 
          id: review:00a0ic854u4j7z02s00v, 
          person: person:67s8cp304i4p6p83q02d, 
          product: product:41v2qv923h3o7s55e96l, 
          rating: 3, review_text: 'repeat exchange meter lawyer academy sunset gang' };



UPDATE artist:00b2pg847d7b8r08t08t 
CONTENT { address: { address_line_1: '816 Cravenny', 
                     address_line_2: NONE, 
                     city: 'Malvern', 
                     coordinates: [9.854998, -0.731803], 
                     country: 'England', post_code: 'HO1D 8WY' }, 
                     company_name: NONE, email: 'acute2086@protonmail.com', 
                     first_name: 'Lasonya', id: artist:00b2pg847d7b8r08t08t, 
                     last_name: 'Spears', name: 'Lasonya Spears', phone: '0800 085411' };


SELECT id,rating, artist.address.city 
AS city, artist.email 
AS email 
FROM review;