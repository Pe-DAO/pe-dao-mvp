import { GoogleMap, Marker, useJsApiLoader } from '@react-google-maps/api';
import { useCallback, useState } from 'react';

import styled from 'styled-components'


export default function MapDiv(){

    const containerStyle = {
        width: '23.3rem',
        height: '37.6rem'
      };
      
      const center = {
        lat: -3.745,
        lng: -38.523
      };

      const { isLoaded } = useJsApiLoader({
        id: 'google-map-script',
        googleMapsApiKey: process.env.REACT_APP_API_GOOGLE
      })
    
      const [map, setMap] = useState(null)
    
      const onLoad = useCallback(function callback(map) {
        const bounds = new window.google.maps.LatLngBounds(center);
        map.fitBounds(bounds);
    
        setMap(map)
      }, [])
    
      const onUnmount = useCallback(function callback(map) {
        setMap(null)
      }, [])
    
      return isLoaded ? (
        
          <GoogleMap
          mapContainerClassName='map-google'
            mapContainerStyle={containerStyle}
            center={center}
            zoom={9}
            
            
          >
            {/* Child components, such as markers, info windows, etc. */ }
          </GoogleMap>
        
      ) : <>Test</>

}


