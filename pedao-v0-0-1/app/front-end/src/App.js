import './App.css';
import styled from 'styled-components';
import { BiHomeAlt2 } from 'react-icons/bi';
import { BsCircleFill , BsArrowUpRight} from 'react-icons/bs';
import { FaWallet } from 'react-icons/fa'
import MapDiv from './components/Map';

function App() {
  return (
    <div className="App">
      
      <div className='Screen'>
      <MapDiv/>
      <Bottom>
        <div>
            <div>
              <BiHomeAlt2/>
              <p>Home</p>
            
            </div>
            <div>
            <BsCircleFill/>
              <p>Cycling</p></div>
            <div>
            <BsArrowUpRight/>
              <p>Collectables</p></div>
            <div>
              <FaWallet/>
              <p>Wallet</p></div>
        </div>
      
        </Bottom>
      </div>
    </div>
  );
}

export default App;



const Bottom = styled.div`
  background-color: #313131;
  div {
    display: flex;
    justify-content: space-around;
    height: 4rem;
    align-items: center;
    
    div {
     color: #20E7B7;

     height: 4rem;
      p {
        color: white;
        overflow-y: hidden;
        font-family: 'Bebas Neue';
        font-size: 0.9rem;
      }
      font-size: 1.6rem;
      display: flex;
      flex-direction: column;
      justify-content: center;
    }
    }
`