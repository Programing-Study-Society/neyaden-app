// import { Image } from 'react-bootstrap';
import { Swiper, SwiperSlide } from 'swiper/react';
import 'swiper/css';
import { Navigation, Pagination, A11y, Autoplay } from 'swiper/modules';
import 'swiper/css/navigation';
import 'swiper/css/pagination';
import 'swiper/css/a11y';
import 'swiper/css/scrollbar'

type props = {
  slideImageLinks: string[],
  size?: string,
}

export default function MySlider(props: props) {
  let size = props.size !== undefined ? props.size : '75%';
  return (
    <Swiper
      modules={[Navigation, Pagination, A11y, Autoplay]}
      slidesPerView={1}
      spaceBetween={0}
      autoplay={{delay: 5000}}
      centeredSlides
      speed={750}
      loop={true}
      pagination={{ clickable: true }}
      style={{width: `calc(${size} * 2.0)`, height: size, borderRadius: "0.75vmin"}}
    >
      {props.slideImageLinks.map((imageLink, idx) => 
        <SwiperSlide key={idx} style={{height: 'auto', display: 'flex', justifyContent: 'center', alignItems: 'center'}}>
          <img src={imageLink} alt={idx.toString()} style={{objectFit: 'cover', width: '100%', height: '100%'}} />
        </SwiperSlide>
      )}
    </Swiper>
  );
};