// // import archetypes from "../store/archetypeStore";
// import { loadFBX } from "./ropes";

// /**
//  * if archetypes is not loaded, load data from file
//  *
//  * @returns {Promise}
//  */
// export function getDiva () {
//     // if ( archetypes.diva ) {
//     //     return Promise.resolve( archetypes.diva );
//     // }

//     // load the file
//     return new Promise((resolve, reject) => {
//         loadFBX("fbx/taunt.fbx")
//             .then(fbxData => {

//                 // archetypes.diva = fbxData;

//                 resolve(fbxData);
//             })
//             .catch(error => {
//                 reject(error);
//             });
//     });
// }

// // export function getShadow () {
// //     if (archetypes.shadow) {
// //         return Promise.resolve(archetypes.shadow);
// //     }

// //     // load the file
// //     return new Promise((resolve, reject) => {
// //         loadFBX("fbx/shadow.fbx")
// //             .then(fbxData => {

// //                 archetypes.shadow = fbxData;

// //                 resolve(fbxData);
// //             })
// //             .catch(error => {
// //                 reject(error);
// //             });
// //     });
// // }