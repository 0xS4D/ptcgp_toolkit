import MainLayout from '@/views/Layout/Layout';
import HomePage from '@/views/Home/HomePage';
import HeadlessPage from './views/Headless/HeadlessPage';
import ApksPage from './views/Apks/ApksPage';
import GMetaPage from './views/Gmetadata/GMetaPage';
import ProtoPage from './views/ProtoExtractor/ProtoPage';

export const routes = [
  {
    path: '/',
    Component: MainLayout,
    children: [
      { name: 'Home', href: '/', index: true, Component: HomePage },
      { name: 'APKS Extractor', path: 'apks', Component: ApksPage },
      { name: 'GMetadata', path: 'gmetedata', Component: GMetaPage },
      { name: 'Proto Extractor', path: 'protoextractor', Component: ProtoPage },
      { name: 'Headless Client', path: 'headless', Component: HeadlessPage },
    ],
  },
];
